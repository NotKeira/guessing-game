fn exit(exitCount, count) {
    println!("That isn't a correct guess. Enter a number or I will self destruct the code.");
    count += 1;
    exitCount +=1;
    if exitCount >= 2 {
        break;
    } else {
        continue
    }
 }
