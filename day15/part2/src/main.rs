#[derive(Clone)]
struct Lens {
    label: String,
    focal_length: u8,
}

fn make_hash(input: &str) -> u8 {
    let mut value: u8 = 0;
    for char in input.as_bytes() {
        value = value.wrapping_add(*char);
        value = value.wrapping_mul(17);
    }
    value
}

fn print_boxes(boxes_list: &Vec::<Vec::<Lens>>) {
    for (box_index, lens_box) in boxes_list.iter().enumerate() {
        if lens_box.len() == 0 {
            continue;
        }
        print!("Box {}: ", box_index);
        for lens in lens_box {
            print!("[{} {}] ", lens.label, lens.focal_length);
        }
        println!();
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut boxes = vec![Vec::<Lens>::new(); 256];
    for step in input.split(",") {
        let mut last_alpha = 0;
        for character in step.chars() {
            if character.is_alphabetic() {
                last_alpha += 1
            } else {
                break;
            }
        }
        let label = step[0..last_alpha].to_string();
        let operation = step.chars().nth(last_alpha).unwrap();
        let box_index = make_hash(&label) as usize;
        match operation {
            '=' => {
                let focal_length = step[last_alpha + 1..].to_string().parse::<u8>().unwrap();
                if boxes[box_index].iter().filter(|x| x.label == label).count() == 1 {
                    let mut lens_index = 0;
                    for (index, lens) in boxes[box_index as usize].iter().enumerate() {
                        if lens.label == label {
                            lens_index = index;
                        }
                    }
                    boxes[box_index][lens_index].focal_length = focal_length;
                } else {
                    boxes[box_index].push(Lens{label: label.to_string(), focal_length: focal_length});
                }
            }
            '-' => {
                if boxes[box_index].iter().filter(|x| x.label == label).count() == 1 {
                    let mut lens_index = 0;
                    for (index, lens) in boxes[box_index as usize].iter().enumerate() {
                        if lens.label == label {
                            lens_index = index;
                        }
                    }
                    boxes[box_index].remove(lens_index); 
                }
            }
            _ => {
                
            }
        }
        println!("After \"{}\"", step);
        print_boxes(&boxes);
    }
    let mut focusing_power = 0;
    for (box_index, lens_box) in boxes.iter().enumerate() {
        for (lens_index, lens) in lens_box.iter().enumerate() { 
            focusing_power += (box_index + 1) * (lens_index + 1) * (lens.focal_length as usize);
        }
    }
    println!("{}", focusing_power);
}