use std::{sync::mpsc, thread, ops::Range};

use kdam::prelude::*;

use bedrock_finder::{Generator, OldPaperLegacyGenerator};

const THREADS: u32 = 10;  // Recommended to use the number of CPUs you have
const X_RANGE: Range<i32> = -3_750_000..3_750_000;
const Z_RANGE: Range<i32> = -3_750_000..3_750_000;
const MAX_COUNT: u8 = 18;  // Number of `is_bedrock()` checks when calculating `count`
const THRESHOLD: u8 = 2;  // Allowed number of mistakes in match

fn main() {
    let world_seed = 64149200;
    let y = 4;

    // Progress Bar
    let mut pb = tqdm!(
        total = (X_RANGE.len() / 16) * (Z_RANGE.len() / 16),
        colour = "#3498db"
    );
    let (tx, rx) = mpsc::channel();  // Send progress updates

    // Split X_RANGE into multithreaded bands
    let x_range: Vec<i32> = X_RANGE.step_by(16).collect();

    let mut handles = vec![];

    let band_size = (x_range.len() as f64 / THREADS as f64).ceil() as usize;

    for x_band in x_range.chunks(band_size) {
        let x_band = x_band.to_owned();
        let tx = tx.clone();

        handles.push(thread::spawn(move || {
            // Initialize generator
            let generator = OldPaperLegacyGenerator::new(world_seed);

            // Do brute-forcing
            for chunk_x in x_band {
                for chunk_z in Z_RANGE.step_by(16) {
                    // Offsets you are 100% sure of
                    // Tip: Order these if statements so `true` values are on top for faster branching
                    if generator.is_bedrock(chunk_x + 1, y + 0, chunk_z + 1) &&
                    generator.is_bedrock(chunk_x + 3, y + 0, chunk_z + 1) &&
                    generator.is_bedrock(chunk_x + 1, y + 0, chunk_z + 2) &&
                    generator.is_bedrock(chunk_x + 2, y + 0, chunk_z + 2) &&
                    generator.is_bedrock(chunk_x + 3, y + 0, chunk_z + 2) &&
                    generator.is_bedrock(chunk_x + 1, y + 0, chunk_z + 3) &&
                    generator.is_bedrock(chunk_x + 2, y + 0, chunk_z + 3) &&
                    generator.is_bedrock(chunk_x + 3, y + 0, chunk_z + 3) &&
                    generator.is_bedrock(chunk_x + 4, y + 0, chunk_z + 3) &&
                    generator.is_bedrock(chunk_x + 3, y + 0, chunk_z + 4) &&
                    generator.is_bedrock(chunk_x + 4, y + 0, chunk_z + 4) &&
                    generator.is_bedrock(chunk_x + 1, y + 0, chunk_z + 5) &&
                    generator.is_bedrock(chunk_x + 2, y + 0, chunk_z + 5) &&
                    generator.is_bedrock(chunk_x + 3, y + 0, chunk_z + 5) &&
                   !generator.is_bedrock(chunk_x + 2, y + 0, chunk_z + 1) &&
                   !generator.is_bedrock(chunk_x + 4, y + 0, chunk_z + 1) &&
                   !generator.is_bedrock(chunk_x + 4, y + 0, chunk_z + 2) &&
                   !generator.is_bedrock(chunk_x + 1, y + 0, chunk_z + 4) &&
                   !generator.is_bedrock(chunk_x + 2, y + 0, chunk_z + 4) &&
                   !generator.is_bedrock(chunk_x + 4, y + 0, chunk_z + 5) {

                        // NOTE: Make sure to update MAX_COUNT with the number of checks in this variable below:
                        let count = !generator.is_bedrock(chunk_x + 11, y + 0, chunk_z + 12) as u8 +
                            !generator.is_bedrock(chunk_x + 1, y + 0, chunk_z + 0) as u8 +
                            !generator.is_bedrock(chunk_x + 2, y + 0, chunk_z + 0) as u8 +
                            !generator.is_bedrock(chunk_x + 3, y + 0, chunk_z + 0) as u8 +
                            !generator.is_bedrock(chunk_x + 4, y + 0, chunk_z + 0) as u8 +
                            !generator.is_bedrock(chunk_x + 0, y + 0, chunk_z + 1) as u8 +
                            !generator.is_bedrock(chunk_x + 5, y + 0, chunk_z + 1) as u8 +
                            !generator.is_bedrock(chunk_x + 0, y + 0, chunk_z + 2) as u8 +
                            !generator.is_bedrock(chunk_x + 5, y + 0, chunk_z + 2) as u8 +
                            !generator.is_bedrock(chunk_x + 0, y + 0, chunk_z + 3) as u8 +
                            !generator.is_bedrock(chunk_x + 5, y + 0, chunk_z + 3) as u8 +
                            !generator.is_bedrock(chunk_x + 0, y + 0, chunk_z + 4) as u8 +
                            !generator.is_bedrock(chunk_x + 5, y + 0, chunk_z + 4) as u8 +
                            !generator.is_bedrock(chunk_x + 0, y + 0, chunk_z + 5) as u8 +
                            !generator.is_bedrock(chunk_x + 5, y + 0, chunk_z + 5) as u8 +
                            !generator.is_bedrock(chunk_x + 1, y + 0, chunk_z + 6) as u8 +
                            !generator.is_bedrock(chunk_x + 2, y + 0, chunk_z + 6) as u8 +
                            !generator.is_bedrock(chunk_x + 3, y + 0, chunk_z + 6) as u8 +
                            !generator.is_bedrock(chunk_x + 4, y + 0, chunk_z + 6) as u8;
        
                        if count >= MAX_COUNT - THRESHOLD {
                            println!("Found: ({chunk_x}, {chunk_z}) with {count}/{MAX_COUNT} matches");
                        }
                    }
                }

                tx.send((Z_RANGE.end - Z_RANGE.start) as usize / 16).unwrap();  // Update by length completed
            }
        }));
    }

    // Update progress bar
    for update in rx {
        pb.update(update);
        if pb.completed() {
            break;
        }
    }
    
    // Wait for all to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Debugging:
    // for z in 0..0+16 {
    //     for x in 0..0+16 {
    //         if generator.is_bedrock(x, y, z) {
    //             print!("██")
    //         } else {
    //             print!("  ")
    //         }
            
    //     }
    //     println!();
    // }
    // println!("Result: {}", generator.is_bedrock(4, 4, 3));  // <-- For tracing with debugger
}
