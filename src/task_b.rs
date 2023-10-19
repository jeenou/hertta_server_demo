use std::sync::mpsc;
use crate::Task;

pub fn task_b_sender(task: mpsc::Sender<Task>) {

    task.send(Task::QuitProcess).expect("Failed to send from A to B");
}

pub fn task_b_receiver(task: mpsc::Receiver<Task>) {

    let data = task.recv().expect("Failed to receive from B to A");
        
        match data {
            Task::StartProcess => {
                // Data is equal to Task::StartProcess
                println!("Received StartProcess");
            }
            Task::QuitProcess => {
                // Data is equal to Task::StartProcess
                println!("Received QuitProcess");
            }
        }

}