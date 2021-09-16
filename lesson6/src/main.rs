use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write as IoWrite;
use std::io::{prelude::*, stdin, Lines, Stdin, StdinLock};
use std::io::{Result, Write};
use std::fs::OpenOptions;
use std::io::SeekFrom;
use std::io::Cursor;
use std::process::{Command,Stdio};


fn main() {
    //println!("Please type a number");

    let stdin: Stdin = stdin();
    /*   let reader:StdinLock = stdin.lock();
    let lines: Lines::<StdinLock> = reader.lines();

    for s in lines {
        println!(" lines {} " , s.unwrap_or("error".to_string()));
    }*/
    /*let mut buf = String::new();
    let r = stdin.read_line(&mut buf);

    match r {
        Ok(x) => println!("value read is {}" , buf) ,
        Err(_) => panic!("  could not read ")

    }*/

    let mut w = Vec::new();
    let mut s = String::new();
    // writeln!(&mut w);
    writeln!(&mut s, "{} {} {}", "abcg", 123u32, "karima");
    writeln!(&mut w, "s = {:?}", s);
    println!(" {} ", s);
    println!(" {:?} ", w);

    writeln!(io::stdout(), "error : world not helloable");

    let mut writer = File::create("newfile.txt").unwrap();
    let mut writerbuf = File::create("newfilebuf.txt").unwrap();
    let mut buffer = BufWriter::new(writerbuf);
    writer.write(b"jawaher");
    buffer.write(b"khoukha");
    writer.write_all(b"Karimaa");

    let res = buffer.flush();
    match res {
        Ok(r) => println!("res {:?}", r),
        Err(e) => println!("err {}", e),
    }

    let log = OpenOptions::new().append(true).open("newfile.txt");
    println!("{:?}",log);

   // let fl = OpenOptions::new().write(true).create_new(true).open("newfile3.txt");
//println!("{:?}",fl);

    //let res= writer.seek(SeekFrom::Start(0));
    
    


  let mut file2 = File::open("/Users/karima/trainingrust/lesson5/poem.txt").unwrap();
   println!("ici");

   file2.seek(SeekFrom::Start(210));
   //file2.seek(SeekFrom::Current(-4));
   //file2.seek(SeekFrom::End(-8));
  let mut buffer = Vec::new();
    let f = file2.read_to_end(&mut buffer);

    for (i,j) in buffer.iter().enumerate() {
        println!(" {}  {}", i,j);
    }  
    println!("--------------cursor--------------");
 let  buff =Cursor::new(vec![1,2,3,4]);
 let vec :Vec<u32>=buff.into_inner();
 println!(" {:?}  ", vec);
 let mut buff1 =Cursor::new(vec![1,2,3,4]);
 let  refvec :&Vec<u32> =buff1.get_ref();
 let mut buff2 =Cursor::new(vec![1,2,3,4]);
 let vecmut=buff2.get_mut();

 println!(" {:?}  ", refvec);
 //let mut child = Command::new("ls").arg("-a").arg("e").stdin(Stdio::piped());

 Command::new("ls")
 .arg("-l")
 .arg("-a")
 .spawn()
 .expect("ls command failed to start");

 
}
