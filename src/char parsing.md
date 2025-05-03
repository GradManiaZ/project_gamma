
# This is char parsing

## Why is this even nessesary? *Idiot proofing!*

Parsing is initialised to store a `buffer` of chars as it parses through the input string, a `level_count` to maintain order, and a `last_level` variable which will collapse extra added levels. (eg lvl 2 followed by 5 will be brought down to 3).

> Excersise
>> Workout
>>> Arms
>>>>>>>> Control the molecules (This is rediculous)
>>>>>>>
>> Groceries
>>> eggs

```rust

if last_level == 0 {
  last_level = 1;
}else if last_level< level_count{
  last_level = last_level +1;
}else { // 1 greater or less than
  last_level = level_count;
}
```

This little piece of logic control how the last level paramater is shifted. Creating a list of subtasks makes sense.

This allows the user to add multiple tasks to any project while separating tasks and subtasks.
Rather than apearing as a single block of tasks there is now potential to subdivide task.

![Flush](../assets\char_parsing\char_parsing_1.png)

While looping through the input task string `>` characters denote new levels. Each time this char is encountered, the state is checked, the buffer is flushed to the set of tasks along with its assigned level and the buffer is cleared.

### **Why is the input vec mutable?** *- Preventing code repetition*

Adding `>` to the input stack forces the algorithm to flush the buffer at the end.

![chaos](../assets\char_parsing\char_parsing_2.png)

Other characters are simply added to the buffer string as the message. This ladder algorithm simple requires subsiquent tasks to be no larger than one greater than the previous. Any greater levels are simply ommitted.

### Example Chaos

This feature is meant to prevent this particular case.

```txt
> Set up project structure
>> Define core requirements
>>> Design initial UI mockups
>>>> Implement basic functionality
>>>>>>>>> Write unit tests        <----
>> Review and refactor code
>>> Deploy to staging environment

Project 1: Project: [1]:'First Project' by "Leader A" due Tomorrow [high]
  - Set up project structure
  -- Define core requirements
  --- Design initial UI mockups
  ---- Implement basic functionality
  ----- Write unit tests
  -- Review and refactor code
  --- Deploy to staging environment
```

Where it struggles is incorrect by one decrement errors. It will still maintain a staircase from the start of the project. Potentially idiot proofing is more redundant and giving the user full control is more benificial.

```txt
>>>> Set up project structure
>>> Define core requirements
>> Design initial UI mockups
>>>> Implement basic functionality
>>>>>>>>> Write unit tests
>> Review and refactor code
>>> Deploy to staging environment

Project 2: Project: [2]:'Second Project' by "Leader B" due Tuesday [low]
  - Set up project structure
  -- Define core requirements
  -- Design initial UI mockups
  --- Implement basic functionality
  ---- Write unit tests
  -- Review and refactor code
  --- Deploy to staging environment
```

This staircase shows the worse possible scenario, once it reaches a value where the last count and current count are one appart it assumes everything is in order, which technically it should be.

```txt
>>>>>>> Set up project structure
>>>>>> Define core requirements
>>>>> Design initial UI mockups
>>>> Implement basic functionality
>>> Write unit tests
>> Review and refactor code
> Deploy to staging environment

Project 3: Project: [3]:'Second Project' by "Leader B" due Tuesday [low]
  - Set up project structure
  -- Define core requirements
  --- Design initial UI mockups
  ---- Implement basic functionality
  --- Write unit tests
  -- Review and refactor code
  - Deploy to staging environment
```

I maintain to include the idiot proofing, however flawed.

### Source code `Some(mut task_string)=>`

``` Rust
// maintianing level persistance
let mut last_level: usize = 0;
let mut level_count:usize = 0;
let mut partitioned_tasks: Vec<(usize, String)> = Vec::new();
let mut buff:String = String::new();
task_string.push('>');  // allows all operations to happenwithin the loop;
let task_string = task_string.chars().peekable();
// states: selecting level | writing task
let mut selecting_level:bool = true; // was the last char nota '>'
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
```
