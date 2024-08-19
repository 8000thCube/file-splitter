/// the capacity in bytes we use for buffering to be fast but not eat all the ram
const BUFFER_CAPACITY:usize=1000000000;
/// the main function
fn main()->IOResult<()>{
	let args:Vec<String>=args().collect();
	let user_error=||{
		eprintln!("Usage: {} <[merge/split]> <file> <number of files to split into? (default: 2, ignored if merge)>",args[0]);
		exit(1)
	};
	let n=match args.len(){3=>2,4=>args[3].parse::<u64>().expect("The number of files to split into should be an integer that fits in 8 bytes."),_=>user_error()};
	let filename=&args[2];
	match args[1].as_str(){"merge"=>merge(filename),"split"=>split(filename,n),_=>Ok(user_error()).map(|_|())}
}/// merges the pieces of a file. Assumes the file pieces are in filename.split_{n}
fn merge(filename:&str)->IOResult<()>{
	let (mergedfile,mut splitfilename)=(File::create_new(filename)?,filename.to_owned());
	let mut write=BufWriter::with_capacity(BUFFER_CAPACITY,mergedfile);
	for i in 0..{
		splitfilename.truncate(filename.len());
		write!(&mut splitfilename,".split_{i}").unwrap();
		let mut splitfile=match File::open(&splitfilename){
			Err(error)=>if error.kind()==IOErrorKind::NotFound{break}else{return Err(error)},Ok(file)=>file
		};
		io_copy(&mut splitfile,&mut write)?;
		remove_file(&splitfilename)?;
	}Ok(())
}/// splits a file into n approximately even pieces. The file pieces are in filename.split_{n}
fn split(filename:&str,n:u64)->IOResult<()>{
	let (file,mut newfilename)=(File::open(filename)?,filename.to_owned());
	let len=file.metadata()?.len();
	let (k,r)=(len/n,len%n);
	let mut read=BufReader::with_capacity(BUFFER_CAPACITY,file);
	for i in 0..n{
		newfilename.truncate(filename.len());
		write!(&mut newfilename,".split_{i}").unwrap();
		let mut newfile=File::create_new(&newfilename)?;
		newfile.set_len(k)?;
		io_copy(&mut (&mut read).take(if i<r{1}else{0}+k),&mut newfile)?;
	}Ok(())
}use std::{
	env::args,fmt::Write,fs::{File,remove_file},io::{BufReader,BufWriter,ErrorKind as IOErrorKind,Read as IORead,Result as IOResult,copy as io_copy},process::exit
};
