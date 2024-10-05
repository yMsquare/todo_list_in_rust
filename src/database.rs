use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write,Seek};
pub struct Record{
    pub id:i32,
    pub content:String,
    pub checked:bool
}
pub struct Database{
    pub file:File
}

impl Database {
    // 打开数据库文件
    pub fn open(filename: &str) -> Database {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(filename)
            .unwrap();
        Database { file }
    }
    //Adding a record only need you to write the record line directly to the file.
    pub fn add_record(&mut self, record : &Record){
        let line = format!("{},{},{}\n",record.id,record.content,record.checked);
        writeln!(self.file,"{}",line).unwrap();      
        println!("📋 Item added:{}",record.content);
    }
    //Reading records need using BufReader.
    //It returns a Vec of Records.
    pub fn read_record(&mut self)-> Vec<Record>{
        let reader = BufReader::new(&self.file);
        reader
            .lines()
            .map_while(Result::ok)
            .filter(|line|!line.is_empty())
            .map(|line|parse_record_line(&line))
            .collect()
    }

    //Checking a record just find the record with id specified and change the 'checked'
    //If id out of range, pl
    //If the record is already checked, println
    // pub fn check_record(&mut self,id:i32){
    //     let line = self.find_line_by_id(id);
    //     match line {
    //         Some((i, rst)) => {
    //             match rst {
    //                 Ok(str) =>{
    //                     let record = read_line(line);
    //                     record.checked = true;
    //                     //如何写回文件对应的行？
    //                     println!(" ✅ Item checked!\n");
    //                 }
    //                 Err(e) =>{

    //                 }
                    
    //             }
    //         }
    //         None => {
    //             println!("No such record: {}", id);
    //         }
    //     } 

    // }
    pub fn check_record(&mut self, id: i32) {
        let line = self.find_line_by_id(id);
        
        match line {
            Some((i, result)) => {
                // 读取源文件内容
                match result{
                    Ok(con)=>{
                        println!("{}",&con);
                        let mut record = parse_record_line(&con);
                        println!("record:{}{}{}",&record.id,&record.content,&record.content);
                        if record.checked{
                            println!("Item already checked!");
                            return;
                        }
                        record.checked = true;

                        let contents = fs::read_to_string(".rododb").unwrap();

                        let new_line = format!("{},{},{}",&record.id,&record.content,&record.checked);
                        let mut lines:Vec<&str> = contents.lines().collect();

                        if i < lines.len(){
                            lines[i] = &new_line;
                        }
                        let new_contents = lines.join("\n");
                        self.file.seek(std::io::SeekFrom::Start(0)).unwrap();
                        self.file.write_all(new_contents.as_bytes()).unwrap();//buf accepts the type of &[u8]
                        self.file.set_len(new_contents.len() as u64).unwrap();
                        println!("✅ Item checked: {}", record.content);
                    }
                    Err(e)=>{
                        println!("Error reading line: {}", e);
                    }
                }
                
            }
            None => {
                println!("No such record: {}", id);
            }
        }
    }
    //find a line by its id.
    //returns an Option type of Result<String,err> or None.
    //the line of item should be in the Result.
    pub fn find_line_by_id(&mut self, id:i32)-> Option<(usize, Result<String,std::io::Error>)>{
        let reader = BufReader::new(&self.file);
        let mut lines = reader.lines().enumerate();

        lines.find(|(_, line)| {
            let record = parse_record_line(line.as_ref().unwrap());
            record.id == id
        })
    }
    pub fn remove_record(&mut self, id: i32) {
    let line = self.find_line_by_id(id);
      // match 匹配判断该行是否存在
      match line {
          Some((i, _result)) => {
            
              // 读取源文件内容
              let contents = fs::read_to_string(".rododb").unwrap();
              // 过滤掉对应的行，这里使用的对应 api 可以查看 Rust 标准库
              let new_contents = contents
                  .lines()
                  .enumerate()
                  .filter(|(j, _)| *j != i)
                  .map(|(_, line)| line)
                  .collect::<Vec<_>>()
                  .join("\n");
              // 将新的内容写入到源文件中
              // 这里使用了 std::io::Seek，需要导入
              self.file.seek(std::io::SeekFrom::Start(0)).unwrap();
              self.file.write_all(new_contents.as_bytes()).unwrap();
              self.file.set_len(new_contents.len() as u64).unwrap();

              println!(" ❌ Item removed!\n");
          }
          None => {
              println!("No such record: {}", id);
          }
      }
  }
    pub fn remove_all_record(&mut self){
        match self.file.set_len(0){
            Ok(_)=>{
               println!("successfully removed all records") ;
            }
            Err(e)=>{
                eprint!("error occurred:{}",e);
            }
        }
    }

}

pub fn parse_record_line(line: &str)->Record{
    let items:Vec<&str> = line.split(',').collect();
    if items.len()==1{
        println!("0!");
       return Record{
        id:0,
        content:"".to_string(),
        checked:false
       }; 
    }
    let checked = items[items.len()-1];
    let content = items[1..items.len()-1].join(",");
    Record {
      id: items[0].parse::<i32>().unwrap(),
      content,
      checked:checked.parse::<bool>().unwrap(),
  }
    
}

//Read a line and return a Record
pub fn read_line(line:String) -> Record{
    let sub :Vec<&str> = line.split(",").collect();
    Record{
        id:sub[0].parse().unwrap_or(1),
        content:sub[1].to_string(),
        checked:sub[2].parse::<bool>().unwrap_or(false)
    }
}