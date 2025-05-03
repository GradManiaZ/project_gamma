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
                if last_level == 0 {
                  last_level = 1;
                  
                }else if last_level + 1< level_count{
                  last_level = last_level +1;
                }else { // 1 greater or less than
                  last_level = level_count;
                }
                // println!("\"{}\" [{:?}]",&buff,&partitioned_tasks.last().unwrap_or(&(0_usize,String::new())));
                partitioned_tasks.push((last_level, buff.clone()));
                buff.clear();
                level_count = 1;
              }else {
                level_count+= 1;
              }
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
      project_id:current_index+1,
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
pub fn wrapper(args:&mut Vec<String>)
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
  let mut command = String::from("");
  let mut num_projects = 0_u8;
  // println!("Welcome!\nWhere would you like to start?"); // preliminary print
  // println!("commands: New, Help, Exit");

  while !exit{
    if args.is_empty(){
      println!("How would you like to continue?");
      println!("commands: New, Help, Exit\n");      // levels to commands?
      io::stdin()
        .read_line(&mut command)
        .expect("Purpose: Reads user input for time_scheduler menu\n Error: Failed to read line!");
    }else {
      command = args.pop().unwrap();
      dbg!(&command);
    }
    match command
      .trim()
      .to_ascii_lowercase()
    .as_str(){
      "new" => {
        command.clear();
        println!("Project Format:\n[Project Name], [Leader], [Deadline], [Priority: High, Mid, Low], [>tasks >> sub tasks]");
        io::stdin()
      .read_line(&mut command)
      .expect("Purpose: Read input to create project.\n Error: Unable to read line!");
        // format: 

        let partitions:Vec<&str> = command.split(",").collect();
        
        if partitions.len() < 4{
          println!("Insufficient Elements: {:?}", partitions);
        }
        else {
          assert!( partitions.len() >= 4, "Purpose: Creating valid new project.\n Error: Insufficient elements"); //idiot proof
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
        let title_pad = pad.repeat(3);
        let mut help_message = String::from("\n");
        help_message = format!("{help_message}{title_pad}{}","\tNew Project\n");
        help_message = format!("{help_message}{pad}{}","Containers to fuel productivity\n",);
        help_message = format!("{help_message}{pad}{pad}{}","- Create a project with the new command to get started\n",);
        help_message = format!("{help_message}{pad}{pad}{}","- Follow the prompts to continue\n",);
        help_message = format!("{help_message}{title_pad}{}","\tExit");
        println!("{help_message}");
      },
      "test" =>{
        let test_pad = "#".repeat(8);
        println!("\n{test_pad}INITIALISING TEST {test_pad}\n");

        let mut task_list = String::new();
        task_list = format!("{task_list}{}","> Set up project structure");
        task_list = format!("{task_list}{}",">> Define core requirements");
        task_list = format!("{task_list}{}",">>> Design initial UI mockups");
        task_list = format!("{task_list}{}",">>>> Implement basic functionality");
        task_list = format!("{task_list}{}",">>>>>>>>> Write unit tests");
        task_list = format!("{task_list}{}",">> Review and refactor code");
        task_list = format!("{task_list}{}",">>> Deploy to staging environment");

        let (proj1, num_projects) = Project::new(
          num_projects,
          "First Project".into(),
          "Leader A".into(),
          "Tomorrow".into(),
          "high".into(),
          Some(task_list.clone()),
        );
        let mut task_list = String::new();
        task_list = format!("{task_list}{}",">>>> Set up project structure");
        task_list = format!("{task_list}{}",">>> Define core requirements");
        task_list = format!("{task_list}{}",">> Design initial UI mockups");
        task_list = format!("{task_list}{}",">>>> Implement basic functionality");
        task_list = format!("{task_list}{}",">>>>>>>>> Write unit tests");
        task_list = format!("{task_list}{}",">> Review and refactor code");
        task_list = format!("{task_list}{}",">>> Deploy to staging environment");

        let (proj2, num_projects) = Project::new(
          num_projects,
          "Second Project".into(),
          "Leader B".into(),
          "Tuesday".into(),
          "low".into(),
          Some(task_list),
        );

        let mut task_list = String::new();
        task_list = format!("{task_list}{}",">>>>>>> Set up project structure");
        task_list = format!("{task_list}{}",">>>>>> Define core requirements");
        task_list = format!("{task_list}{}",">>>>> Design initial UI mockups");
        task_list = format!("{task_list}{}",">>>> Implement basic functionality");
        task_list = format!("{task_list}{}",">>> Write unit tests");
        task_list = format!("{task_list}{}",">> Review and refactor code");
        task_list = format!("{task_list}{}","> Deploy to staging environment");

        let (proj3, num_projects) = Project::new(
          num_projects,
          "Second Project".into(),
          "Leader B".into(),
          "Tuesday".into(),
          "low".into(),
          Some(task_list),
        );
        
        let projects: Vec<Project> = vec![proj1,proj2,proj3];

        for (index,proj) in projects.iter().enumerate(){
          println!("Project {}: {}\n",index + 1, *proj);

        }
        // println!("Project 2: {}\n",proj2);
        println!("{test_pad}TESTING COMPLETE{test_pad}\n")
      },
      _=>
      {
        println!("I'm sorry I didn't catch that. Try Help for more options");
      }
    }
    
    command.clear();
  }
}