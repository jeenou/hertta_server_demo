
mod task_a;
mod task_b;

use std::sync::mpsc;
use std::thread;
use task_a::task_a_sender; // Import the function
use task_a::task_a_receiver; // Import the function
use task_b::task_b_sender; // Import the function
use task_b::task_b_receiver; // Import the function

pub enum Task {
    StartProcess, // A task to start a process
    QuitProcess,  // A task to quit a process
}

fn main() {


    //Make a channel to send data from task_a to task_b

    let (tx_ab, rx_ab): (mpsc::Sender<Task>, mpsc::Receiver<Task>) = mpsc::channel();

    //Make a channel to send data from task_b to task_a

    let (tx_ba, rx_ba): (mpsc::Sender<Task>, mpsc::Receiver<Task>) = mpsc::channel();

    //Send task from task_a to task_b

    let thread_a_sender = thread::spawn(|| task_a_sender(tx_ab));
    let thread_b_receiver = thread::spawn(|| task_b_receiver(rx_ab));

    //Send task from task_b to task_a

    let thread_b_sender = thread::spawn(|| task_b_sender(tx_ba));
    let thread_a_receiver = thread::spawn(|| task_a_receiver(rx_ba));

    //the .join() method is used to synchronize the main thread with the spawned threads, ensuring that all the tasks 
    //being performed in those threads have completed before the main thread continues its execution. It's a way to 
    //coordinate and wait for the threads to finish their work.

    let _ = thread_a_sender.join(); 
    let _ = thread_b_receiver.join();

    let _ = thread_b_sender.join(); 
    let _ = thread_a_receiver.join();
}