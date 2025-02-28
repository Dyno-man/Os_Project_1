//Create functions for threading operations
//This way we can get around the mem safety features and not stretch the variables
//thin, this way everything is put inside of a scope properly and the variables are brought in,
//cloned, and then spit back out. We only change the

//Phase 1 Function
//Inputs: Takes in library and users vector
//Outputs: nothing
//Step 1: Create 100 threads for each: borrowing books, returning books
//Step 2: After pushing these threads into their own vectors join all threads together
//After all threads have run exit the function

//Phase 2 Function
//Inputs: Takes in library and users vectors
//Outputs: Nothing
//This phase will essentially be a copy of phase 1 but mutex will be implemented to lock
//out other threads from accessing the library at the same time

//Phase 3 Function
//Inputs: Takes in library and users vectors
//Outputs: Nothing
//Notes: We need to create a scenario where multiple people are trying to access the same resource
//or something else that will allow us to create thread locking

//Phase 4 Function
//Inputs: Takes in library and users vectors
//Outputs: Nothing
//Notes: We need to resolve the scenario in phase 3 by some means


mod book;
mod book_shipment_log;
mod book_action_logger;

use book::{Book, User, fill_book_data, fill_user_data};
use book_shipment_log::*;
use book_action_logger::*;

use rand::{Rng};
use std::io::{stdin};
use std::thread;
use std::thread::*;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};


fn main() {
    // Create some dummy data.
    let library = match new_book_shipment() {
        Ok(library) => library,
        Err(e) => {
            eprintln!("{e}");
            return
        },
    };

    let users = vec![
        User { username: "Alice".into(), loaned_books: Vec::new() },
        User { username: "Bob".into(), loaned_books: Vec::new() },
        User { username: "Charlie".into(), loaned_books: Vec::new() },
    ];

    // Run each phase.
    phase_1(&mut library.clone(), &mut users.clone());
    phase_2(library.clone(), users.clone());
    phase_3(library.clone(), users.clone());
    phase_4(library.clone(), users.clone());
}


// Returns a random number in the range [min, max).
fn random_range(min: usize, max: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

/// Try to lock a mutex, but return None if the lock isn’t acquired within `timeout`.
fn try_lock_with_timeout<T>(
    mutex: &Mutex<T>,
    timeout: Duration
) -> Option<std::sync::MutexGuard<T>> {
    let start = Instant::now();
    loop {
        match mutex.try_lock() {
            Ok(guard) => return Some(guard),
            Err(_) => {
                if start.elapsed() >= timeout {
                    return None;
                }
                sleep(Duration::from_millis(10));
            }
        }
    }
}

//---------------------
// Phase 1: Basic Thread Operations (No Mutex Protection)
//
// In this phase, we spawn 100 threads for borrowing books and 100 threads for returning books.
// To “get around” the borrow-checker issues, we clone the library and user vectors into each thread,
// so that each thread works on its own copy. (In a real-world scenario, this wouldn’t update the original data.)
//---------------------
fn phase_1(lib: &mut Vec<Book>, memb: &mut Vec<User>) {
    println!("--- Phase 1: Basic Thread Operations ---");

    // Borrowing phase
    for _ in 0..100 {
        // Capture indices to operate on.
        let book_index = random_range(0, lib.len());
        let user_index = random_range(0, memb.len());

        // Spawn a thread that borrows a book.
        unsafe{
        scope(|s| {
            s.spawn(|| {
                if !lib[book_index].borrowed {
                    memb[user_index].add_book(&mut lib[book_index]);
                    println!(
                        "Phase 1: User '{}' borrowed book '{}'",
                        memb[user_index].username,
                        lib[book_index].title
                    );
                }
            })
                .join()
                .expect("Thread panicked");
        });}
    }

    // Returning phase
    for _ in 0..100 {
        let user_index = random_range(0, memb.len());
        thread::scope(|s| {
            s.spawn(|| {
                if !memb[user_index].loaned_books.is_empty() {
                    memb[user_index].remove_return_book();
                    println!(
                        "Phase 1: User '{}' returned a book",
                        memb[user_index].username
                    );
                }
            })
                .join()
                .expect("Thread panicked");
        });
    }

    println!("--- End Phase 1 ---\n");
}

//---------------------
// Phase 2: Resource Protection with Mutexes
//
// Here we wrap the library and user vectors in an Arc<Mutex<…>> so that threads can safely
// modify shared data concurrently. We spawn 100 threads that each perform a borrowing and returning operation.
//---------------------
fn phase_2(lib: Vec<Book>, memb: Vec<User>) {
    println!("--- Phase 2: Resource Protection with Mutex ---");

    let lib = Arc::new(Mutex::new(lib));
    let memb = Arc::new(Mutex::new(memb));

    thread::scope(|s| {
        let mut handles = Vec::new();

        // Spawn 100 threads that do both borrowing and returning.
        for _ in 0..100 {
            let lib_clone = Arc::clone(&lib);
            let memb_clone = Arc::clone(&memb);

            handles.push(s.spawn(move || {
                // Borrowing operation:
                {
                    let mut lib_guard = lib_clone.lock().unwrap();
                    let mut memb_guard = memb_clone.lock().unwrap();
                    let book_index = random_range(0, lib_guard.len());
                    let user_index = random_range(0, memb_guard.len());

                    if !lib_guard[book_index].borrowed {
                        memb_guard[user_index].add_book(&mut lib_guard[book_index]);
                        println!(
                            "Phase 2: (Mutex) User '{}' borrowed book '{}'",
                            memb_guard[user_index].username,
                            lib_guard[book_index].title
                        );
                    }
                }

                // Returning operation:
                {
                    let mut memb_guard = memb_clone.lock().unwrap();
                    let user_index = random_range(0, memb_guard.len());
                    if !memb_guard[user_index].loaned_books.is_empty() {
                        memb_guard[user_index].remove_return_book();
                        println!(
                            "Phase 2: (Mutex) User '{}' returned a book",
                            memb_guard[user_index].username
                        );
                    }
                }
            }));
        }

        // Join all threads.
        for handle in handles {
            let _ = handle.join();
        }
    });

    println!("--- End Phase 2 ---\n");
}

//---------------------
// Phase 3: Deadlock Creation
//
// In this phase we intentionally create a deadlock scenario by having two threads lock
// shared resources (the library and the users vector) in different orders.
//---------------------
fn phase_3(lib: Vec<Book>, memb: Vec<User>) {
    println!("--- Phase 3: Deadlock Creation with Timeout ---");

    let lib = Arc::new(Mutex::new(lib));
    let memb = Arc::new(Mutex::new(memb));

    let timeout = Duration::from_secs(2);

    scope(|s| {
        // Thread A: Lock library first, then try to lock members with a timeout.
        let lib_a = Arc::clone(&lib);
        let memb_a = Arc::clone(&memb);
        let handle_a = s.spawn(move || {
            let _lib_guard = lib_a.lock().unwrap();
            println!("Phase 3: Thread A locked library");
            // Simulate work
            thread::sleep(Duration::from_millis(50));
            match try_lock_with_timeout(&memb_a, timeout) {
                Some(_memb_guard) => {
                    println!("Phase 3: Thread A locked members");
                }
                None => {
                    println!("Phase 3: Thread A timed out waiting for members lock");
                }
            }
        });

        // Thread B: Lock members first, then try to lock library with a timeout.
        let lib_b = Arc::clone(&lib);
        let memb_b = Arc::clone(&memb);
        let handle_b = s.spawn(move || {
            let _memb_guard = memb_b.lock().unwrap();
            println!("Phase 3: Thread B locked members");
            thread::sleep(Duration::from_millis(50));
            match try_lock_with_timeout(&lib_b, timeout) {
                Some(_lib_guard) => {
                    println!("Phase 3: Thread B locked library");
                }
                None => {
                    println!("Phase 3: Thread B timed out waiting for library lock");
                }
            }
        });

        let _ = handle_a.join();
        let _ = handle_b.join();
    });

    println!("--- End Phase 3 ---\n");
}


//---------------------
// Phase 4: Deadlock Resolution
//
// This phase resolves the deadlock by enforcing a consistent lock order for all threads.
// Both threads lock the library first, then the members vector.
//---------------------
fn phase_4(lib: Vec<Book>, memb: Vec<User>) {
    println!("--- Phase 4: Deadlock Resolution ---");

    let lib = Arc::new(Mutex::new(lib));
    let memb = Arc::new(Mutex::new(memb));

    thread::scope(|s| {
        // Both threads lock in the same order.
        let lib_a = Arc::clone(&lib);
        let memb_a = Arc::clone(&memb);
        let handle_a = s.spawn(move || {
            let lib_guard = lib_a.lock().unwrap();
            println!("Phase 4: Thread A locked library");
            thread::sleep(Duration::from_millis(50));
            let _memb_guard = memb_a.lock().unwrap();
            println!("Phase 4: Thread A locked members");
        });

        let lib_b = Arc::clone(&lib);
        let memb_b = Arc::clone(&memb);
        let handle_b = s.spawn(move || {
            let lib_guard = lib_b.lock().unwrap();
            println!("Phase 4: Thread B locked library");
            thread::sleep(Duration::from_millis(50));
            let _memb_guard = memb_b.lock().unwrap();
            println!("Phase 4: Thread B locked members");
        });

        let _ = handle_a.join();
        let _ = handle_b.join();
    });

    println!("--- End Phase 4 ---\n");
}