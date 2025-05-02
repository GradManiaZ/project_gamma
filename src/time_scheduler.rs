use std::{fmt, io::{self, Write}};


pub struct Project{
  project_id    : u8,   // unlikely to cross 255u8
  project_name  : String,
  
  leader        :String,
  deadline      :String,
  prio          :String,
  tasks         :Option<Vec<(usize,String)>>,

}

impl Project
{
  pub fn new(
    current_index: u8,
    proj_name: String,
    leader:String,
    deadline:String,
    prio:String,
    tasks:Option<String>,
  ) -> (Self,u8)
  {
    // ensure overflow check
    assert!(current_index < 255, "Current index has exceeded 255 capacity");


    // let numbers = [1, 2, 3, 4, 5];

    // let zero = "0".to_string();

    // let result = numbers.iter().fold(zero, |acc, &x| {
    //     format!("({acc} + {x})")
    // });

    let tasks:Option<Vec<(usize,String)>> = match tasks {

      Some(mut task_string)=> {
        // parsing all the bs,
        // maintianing level persistance
        let mut last_level: usize = 0;
        let mut level_count:usize = 0;
        let mut partitioned_tasks: Vec<(usize, String)> = Vec::new();
        let mut buff:String = String::new();
        task_string.push('>');  // allows all operations to happen within the loop;
        let task_string = task_string.chars().peekable();

        // states: selecting level | writing task
        let mut selecting_level:bool = true; // was the last char not a '>'
        for char in task_string{
          match char{
            '>' => { // character defines depth
              //define depth
              //flush buffer
              //clear buffer
              //reset level count
              if !selecting_level{
                last_level = level_count;
                if last_level == 0 {
                  partitioned_tasks.push((1, buff.clone()));
                  buff.clear();
                }else {
                  partitioned_tasks.push((last_level, buff.clone()));
                  buff.clear();
                  level_count = 0;
                }
              }
              level_count+= 1;
              selecting_level = true;
            },
            _ => {
              selecting_level = false; 
              buff.push(char);
            }
          }
        }
        Some(partitioned_tasks)
      },
      None => {
        None 
        
      }
    };

    let new_proj = Project{
      project_id:current_index,
      project_name:proj_name,
      leader        :leader,
      deadline      :deadline,
      prio          :prio,
      tasks         :tasks,
    };
    (new_proj,(current_index+1))

  }
}

//project_id:current_index,
// project_name:proj_name,
// leader        :leader,
// deadline      :deadline,
// prio          :prio,
// tasks         :tasks,
// >   One level
// >>  Two levels
// >>> Three levels
// >>  Two levels

// conditions, if the first task doesn't have a task indicator add one
// any tasks without stares colapse into higher levels
impl fmt::Display for Project{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let pad = " ".repeat(2);
    let mut temp = String::new();
    let task_breakdown = match &self.tasks{
      Some(x) => {
        for (depth,description) in x{
          temp = format!("{temp}\n{pad}{}{}","-".repeat(*depth),*description);
        }
        temp
      },
      None => {
        temp
      }
    };
    let mut msg = format!("Project: [{}]:\'{}\' by \"{}\" due {} [{}]{}", self.project_id,self.project_name, self.leader,self.deadline,self.prio,task_breakdown);


    write!(f,"{}", msg)
  }
}



/// This is a CLI interface for the time_scheduler module
pub fn menu()
{
  //const BLANK:u8 = 0b00000001;  // Here by using a while 9 bytes anyway?
  //const BLANK:u8 = 0b00000010;  // Here by using a while 9 bytes anyway?
  //const BLANK:u8 = 0b00000100;  // Here by using a while 9 bytes anyway?
  //const BLANK:u8 = 0b00001000;  // Here by using a while 9 bytes anyway?
  //const BLANK:u8 = 0b00010000;  // Here by using a while 9 bytes anyway?
  //const BLANK:u8 = 0b00100000;  // Here by using a while 9 bytes anyway?
  //const BLANK:u8 = 0b01000000;  // Here by using a while 9 bytes anyway?
  //const BLANK:u8 = 0b10000000;  // Here by using a while 9 bytes anyway?



  let pad = " ".repeat(2);
  let mut exit = false;
  let mut user_input = String::from("");
  let mut num_projects = 0_u8;
  println!("Welcome!\nWhere would you like to start?"); // preliminary print
  println!("Commands: New Project, Help, Exit");

  while !exit{
    io::stdin()
      .read_line(&mut user_input)
      .expect("Purpose: Reads user input for time_scheduler menu\n Error: Failed to read line!");
    
    match user_input
      .trim()
      .to_ascii_lowercase()
    .as_str(){
      "new project" => {
        user_input.clear();
        println!("Project Format:\n[Project Name], [Leader], [Deadline], [Priority: High, Mid, Low], [>tasks >> sub tasks]");
        io::stdin()
      .read_line(&mut user_input)
      .expect("Purpose: Read input to create project.\n Error: Unable to read line!");
        // format: 

        let partitions:Vec<&str> = user_input.split(",").collect();
        
        if partitions.len() < 4{
          println!("Insufficient Elements: {:?}", partitions);
        }
        else {
          assert!( partitions.len() >= 4, "Purpose: Creating valid new project.\n Error: Insufficient elements");
          let proj_name =  partitions[0].to_owned();
          let lead_name =  partitions[1].to_owned();
          let dead_line =  partitions[2].to_owned();
          let prio      =  partitions[3].to_owned();
          let tasks:Option<String> = None;
          if partitions.len() > 4{
            assert!( partitions.len() >= 5, "Purpose: Creating valid new project.\n Error: Insufficient elements");
            let tasks     =  Some(partitions[4].to_owned());
          }
          let (new_proj,num_projects) = self::Project::new(num_projects, proj_name, lead_name, dead_line, prio, tasks);

          println!("Project created: {}\n", new_proj);
          // view project? 
            
        }
        println!("How would you like to continue?");
      },
      "secret menu option"=>{
        println!("╭(ʘ̆~◞౪◟~ʘ̆)╮");
      },
      "exit" => {
        // save
        print!("\x1B[2J\x1B[1;1H"); //clear terminal
        std::io::stdout().flush().unwrap(); // push changes to terminal
        println!("Have a nice day!");
        exit = true;
      },
      "help" => 
      {
        let help_message = format!(
              "\tNew Project\n
          {pad}- Container to fuel productivity\n
          {pad}- Create a project with the new project command to get started\n
          {pad}- Follow the following prompts to continue\n
              \tExit",);
        println!("{help_message}");
      },
      "test" =>{
        num_projects += 1;
        let proj = Project{
          project_id:0,
          project_name:"Name of proj".into(),
          leader:"me".into(),
          deadline:"tomorrow".into(),
          prio:"high".into(),
          tasks:Some(
            vec![
              (0,"Set up project structure".into()),
              (1,"Define core requirements".into()),
              (2,"Design initial UI mockups".into()),
              (3,"Implement basic functionality".into()),
              (4,"Write unit tests".into()),
              (1,"Review and refactor code".into()),
              (2,"Deploy to staging environment".into()),
            ]
          )
        };

        println!("{}",proj);
      },
      _=>
      {
        println!("I'm sorry I didn't catch that. Try Help for more options");
      }
    }
    println!("How would you like to continue?");
    user_input.clear();
  }
}