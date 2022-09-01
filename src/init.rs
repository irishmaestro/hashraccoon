use std::io::{stdout, Write};
use std::{time, thread};

pub fn initialize() {
    let hash_raccoon = r"    @@@  @@@  @@@@@@   @@@@@@ @@@  @@@      @@@@@@@   @@@@@@   @@@@@@@  @@@@@@@  @@@@@@   @@@@@@  @@@  @@@
    @@!  @@@ @@!  @@@ !@@     @@!  @@@      @@!  @@@ @@!  @@@ !@@      !@@      @@!  @@@ @@!  @@@ @@!@!@@@
    @!@!@!@! @!@!@!@!  !@@!!  @!@!@!@!      @!@!!@!  @!@!@!@! !@!      !@!      @!@  !@! @!@  !@! @!@@!!@!
    !!:  !!! !!:  !!!     !:! !!:  !!!      !!: :!!  !!:  !!! :!!      :!!      !!:  !!! !!:  !!! !!:  !!!
     :   : :  :   : : ::.: :   :   : :       :   : :  :   : :  :: :: :  :: :: :  : :. :   : :. :  ::    : 
                                                                                                          ";
    println!("");
    println!("{}", hash_raccoon);
    print_seq("🦝 Initializing hashraccoon");
}

#[allow(unused)]
pub fn print_seq(txt: &str) {
    for i in txt.chars() {
        print!("{}", i);
        stdout().flush();
        thread::sleep(time::Duration::from_millis(10));
    };
    println!();
}