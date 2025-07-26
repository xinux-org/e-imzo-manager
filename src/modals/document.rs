use eimzo::get_pfx_files_in_folder;
use relm4::Worker;
use relm4::ComponentSender;
use std::fs;
use std::path::PathBuf;
use crate::pages::select_mode::SelectModeMsg;
// use crate::pages::select_mode::SelectModeMsg;

/// The document is a headless component which holds and manages the data model.
/// It receives input events FROM the App to update the data model.
/// When updates to the model occur, it sends output events TO the App.
///
/// The document's interface is just input and output events. As a result you have a lot of freedom
/// in how you choose to store the data model within the component, which backing store you use
/// (such as the file system, a database, or a Web API), and how you synchronise to the backing
/// store (e.g. manual save/load control, auto-saving on each change, batching up changes before
/// syncing, and so on).
pub struct Document {}

#[derive(Default)]
struct TagModel {
    name: String,
}
#[derive(Default)]
struct TaskModel {
    name: String,
    tags: Vec<TagModel>,
}
#[derive(Default)]
struct Model {
    tasks: Vec<TaskModel>,
}

#[derive(Debug)]
pub enum DocumentInput {
    // extra operations on the document itself (in this case, related to file I/O)
    Open(PathBuf),
    Save(PathBuf),
    // events related to the model that the document stores
    // Clear,
    // AddTask,
    // DeleteTask(DynamicIndex),
    // ChangeTaskName(DynamicIndex, String),
    // AddTag(DynamicIndex, String),
    // DeleteTag(DynamicIndex, DynamicIndex),
}

#[derive(Debug)]
enum DocumentOutput {
    Cleared,
    AddedTask,
    DeletedTask(usize),
    ChangedTaskName(usize, String),
    AddedTag(usize, String),
    DeletedTag(usize, usize),
}

impl Worker for Document {
    type Init = ();
    type Input = DocumentInput;
    type Output = SelectModeMsg;

    fn init(_init: Self::Init, _sender: ComponentSender<Self>) -> Self {
        // let model: Model = Model::default();
        Self {}
    }

    fn update(&mut self, input: DocumentInput, sender: ComponentSender<Self>) {
        match input {
            DocumentInput::Save(path) => {
                println!("Save as PFX to {path:?}");
                let copied_file = &path.file_name().unwrap().to_str().unwrap();
                
                match get_pfx_files_in_folder("/media/DSKEYS") {
                    Ok(file_names) => {
                        if file_names.contains(&copied_file.to_string()) {
                            // todo show dialog message that file already exists
                            ()
                        } else {
                            let _ = fs::copy(&path, format!("/media/DSKEYS/{}", copied_file));
                            let _ = sender.output(SelectModeMsg::SaveFile(copied_file.to_string()));
                        }
                    }
                    Err(e) => println!("Error in function eimzo::get_pfx_files_in_folder: {}", e),
                    }
            }








            DocumentInput::Open(path) => {
                println!("Open tasks document at {path:?}");
            } // DocumentInput::Clear => {
              //     self.model.tasks.clear();

              //     let _ = sender.output(DocumentOutput::Cleared);
              // }
              // DocumentInput::AddTask => {
              //     self.model.tasks.push(TaskModel::default());

              //     let _ = sender.output(DocumentOutput::AddedTask);
              // }
              // DocumentInput::DeleteTask(index) => {
              //     self.model.tasks.remove(index.current_index());

              //     let _ = sender.output(DocumentOutput::DeletedTask(index.current_index()));
              // }
              // DocumentInput::ChangeTaskName(index, name) => {
              //     if let Some(task) = self.model.tasks.get_mut(index.current_index()) {
              //         task.name.clone_from(&name);
              //     }

              //     // We don't technically need to send an event, because gtk::Entry updates itself
              //     // this is just to make the example consistent.
              //     let _ = sender.output(DocumentOutput::ChangedTaskName(index.current_index(), name));
              // }
              // DocumentInput::AddTag(task_index, name) => {
              //     if let Some(task) = self.model.tasks.get_mut(task_index.current_index()) {
              //         task.tags.push(TagModel { name: name.clone() })
              //     }

              //     let _ = sender.output(DocumentOutput::AddedTag(task_index.current_index(), name));
              // }
              // DocumentInput::DeleteTag(task_index, tag_index) => {
              //     if let Some(task) = self.model.tasks.get_mut(task_index.current_index()) {
              //         task.tags.remove(tag_index.current_index());
              //     }

              //     let _ = sender.output(DocumentOutput::DeletedTag(
              //         task_index.current_index(),
              //         tag_index.current_index(),
              //     ));
              // }
        }
    }
}
