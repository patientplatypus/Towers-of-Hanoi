

use std::io;


extern crate rand;

use rand::distributions::{IndependentSample, Range};


#[derive(Debug)]
struct Disk {
    size: u32,
}

struct spoketable{

    spoke1: Vec<Disk>,
    spoke2: Vec<Disk>,
    spoke3: Vec<Disk>
}

impl spoketable{

	fn printoutgamestate(&self){
		println!("spoke1: {:?}", self.spoke1);
		println!("spoke2: {:?}", self.spoke2);
		println!("spoke3: {:?}", self.spoke3);
	}

	fn nextiteration (&mut self, lastmove: u32) -> u32{
		//algorithm from wikipedia
		let mut disktomove = 0 as u32;
		let mut movedfromatoc = false;
		let mut movedfromctoa = false;
		let mut movedfromatob = false;
		let mut movedfrombtoa = false;
		let mut movedfrombtoc = false;
		let mut movedfromctob = false;
		let mut lastmovereturn = 0 as u32;


		if self.spoke3[0].size == 1{
			println!("We're all done!!!");
			return 0;
		}

		//if self.spoke3[1].size == 0{

			if lastmove == 3 || lastmove == 0{
		
				//first try and move from the A pole to the C pole
				for x in 0..7{
					if self.spoke1[x].size != 0 {
						disktomove = self.spoke1[x].size;
						
						for z in 0..7{
							if z == 6{
								if self.spoke3[z].size == 0{
									self.spoke3[z].size = disktomove;
									self.spoke1[x].size = 0;
									movedfromatoc = true;
									break;
								}
							}else if z!=6 {
								//println!("the value of the z index is {:?}", z);
								if self.spoke3[z].size == 0 && self.spoke3[z+1].size>disktomove{
									self.spoke3[z].size = disktomove;
									self.spoke1[x].size = 0;
									movedfromatoc = true;
									break;
								}
							}
						}
						if movedfromatoc == true {
							lastmovereturn = 1;
							return lastmovereturn;
						}
					}
				}

				//then try and move from the C pole to the A pole
				for x in 0..7{
					if self.spoke3[x].size != 0 {
						disktomove = self.spoke3[x].size;
						
						for z in 0..7{
							if z == 6{
								if self.spoke1[z].size == 0{
									self.spoke1[z].size = disktomove;
									self.spoke3[x].size = 0;
									movedfromctoa = true;
									break;
								}
							}else if z!=6 {
								if self.spoke1[z].size == 0 && self.spoke1[z+1].size>disktomove{
									self.spoke1[z].size = disktomove;
									self.spoke3[x].size = 0;
									movedfromctoa = true;
									break;
								}
							}
						}
						if movedfromctoa == true {
							lastmovereturn = 1;
							return lastmovereturn;
						}
					}
				}
			}


			if lastmove == 1 {
				//then try and move from the A pole to the B pole
				for x in 0..7{
					if self.spoke1[x].size != 0 {
						disktomove = self.spoke1[x].size;
						
						for z in 0..7{
							if z == 6{
								if self.spoke2[z].size == 0{
									self.spoke2[z].size = disktomove;
									self.spoke1[x].size = 0;
									movedfromatob = true;
									break;
								}
							}else if z!=6 {
								if self.spoke2[z].size == 0 && self.spoke2[z+1].size>disktomove{
									self.spoke2[z].size = disktomove;
									self.spoke1[x].size = 0;
									movedfromatob = true;
									break;
								}
							}
						}
						if movedfromatob == true {
							lastmovereturn = 2;
							return lastmovereturn;
						}
					}
				}

				//then try and move from the B pole to the A pole
				for x in 0..7{
					if self.spoke2[x].size != 0 {
						disktomove = self.spoke2[x].size;
						
						for z in 0..7{
							if z == 6{
								if self.spoke1[z].size == 0{
									self.spoke1[z].size = disktomove;
									self.spoke2[x].size = 0;
									movedfrombtoa = true;
									break;
								}
							}else if z!=6 {
								if self.spoke1[z].size == 0 && self.spoke1[z+1].size>disktomove{
									self.spoke1[z].size = disktomove;
									self.spoke2[x].size = 0;
									movedfrombtoa = true;
									break;
								}
							}
						}
						if movedfrombtoa == true {
							lastmovereturn = 2;
							return lastmovereturn;
						}
					}
				}

			}


			if lastmove == 2{
					//then try and move from the B pole to the C pole
				for x in 0..7{
					if self.spoke2[x].size != 0 {
						disktomove = self.spoke2[x].size;
						
						for z in 0..7{
							if z == 6{
								if self.spoke3[z].size == 0{
									self.spoke3[z].size = disktomove;
									self.spoke2[x].size = 0;
									movedfrombtoc = true;
									break;
								}
							}else if z!=6 {
								if self.spoke3[z].size == 0 && self.spoke3[z+1].size>disktomove{
									self.spoke3[z].size = disktomove;
									self.spoke2[x].size = 0;
									movedfrombtoc = true;
									break;
								}
							}
						}
						if movedfrombtoc == true {
							lastmovereturn = 3;
							return lastmovereturn;
						}
					}
				}

				//println!("right before the c to b move");
				//then try and move from the C pole to the B pole
				for x in 0..7{
					if self.spoke3[x].size != 0 {
						disktomove = self.spoke3[x].size;
						
						for z in 0..7{
							if z == 6{
								if self.spoke2[z].size == 0{
									self.spoke2[z].size = disktomove;
									self.spoke3[x].size = 0;
									movedfromctob = true;
									break;
								}
							}else if z!=6 {
								if self.spoke2[z].size == 0 && self.spoke2[z+1].size>disktomove{
									self.spoke2[z].size = disktomove;
									self.spoke3[x].size = 0;
									movedfromctob = true;
									break;
								}
							}
						}
						if movedfromctob == true {
							lastmovereturn = 3;
							return lastmovereturn;
						}
					}
				}

			}

		//}

		return lastmovereturn;
	
	}

	fn printoutspokes(&self){
		for x in 0..7 {	
			if self.spoke1[x].size == 0{
				println!("   .   ");
			}
			if self.spoke1[x].size == 1{
				println!("   *   ");
			}
			if self.spoke1[x].size == 2{
				println!("   **  ");
			}
			if self.spoke1[x].size == 3{
				println!("   *** ");
			}
			if self.spoke1[x].size == 4{
				println!("   ****");
			}
			if self.spoke1[x].size == 5{
				println!("  *****");
			}
			if self.spoke1[x].size == 6{
				println!(" ******");
			}
			if self.spoke1[x].size == 7{
				println!("*******");
			}
		}

		println!("_______");

		for x in 0..7 {	
			if self.spoke2[x].size == 0{
				println!("   .   ");
			}
			if self.spoke2[x].size == 1{
				println!("   *   ");
			}
			if self.spoke2[x].size == 2{
				println!("   **  ");
			}
			if self.spoke2[x].size == 3{
				println!("   *** ");
			}
			if self.spoke2[x].size == 4{
				println!("   ****");
			}
			if self.spoke2[x].size == 5{
				println!("  *****");
			}
			if self.spoke2[x].size == 6{
				println!(" ******");
			}
			if self.spoke2[x].size == 7{
				println!("*******");
			}
		}

		println!("_______");

		for x in 0..7 {	
			if self.spoke3[x].size == 0{
				println!("   .   ");
			}
			if self.spoke3[x].size == 1{
				println!("   *   ");
			}
			if self.spoke3[x].size == 2{
				println!("   **  ");
			}
			if self.spoke3[x].size == 3{
				println!("   *** ");
			}
			if self.spoke3[x].size == 4{
				println!("   ****");
			}
			if self.spoke3[x].size == 5{
				println!("  *****");
			}
			if self.spoke3[x].size == 6{
				println!(" ******");
			}
			if self.spoke3[x].size == 7{
				println!("*******");
			}
		}

		println!("_______");
	}


}


fn main() {

	//initial game print

    println!("Hello, and welcome!");
    println!("We are going to play towers of hanoi");


    let mut stack1 = Vec::new();
    let mut stack2 = Vec::new();
    let mut stack3 = Vec::new();
    let mut lastmovemain = 0 as u32;

    let mut x = 1;

    loop {
		
	    let mut xvalue = x as u32;

	    println!("hello from the forloop");

	    println!("value of xvalue: {:?}", xvalue);

		let mut newdisk = Disk {
			size: xvalue,
		};

		let mut zerodisk1 = Disk {
			size: 0,
		};

		let mut zerodisk2 = Disk {
			size: 0,
		};

		stack1.push(newdisk);
		stack2.push(zerodisk1);
		stack3.push(zerodisk2);

		x = x + 1;
		if x == 8{
			break;
		}
    }    

    println!("stack 1 from main: {:?}", stack1);
    
    let mut currentgame = spoketable {
    	spoke1 : stack1,
    	spoke2 : stack2,
    	spoke3 : stack3
    };

    currentgame.printoutgamestate();
    currentgame.printoutspokes();

    println!("++++++++++++++++++++++++++++++");


    loop {
    	println!("##############################");
    //	println!("lastmovemain is: {:?}", lastmovemain);
	    lastmovemain = currentgame.nextiteration(lastmovemain);
	    currentgame.printoutspokes();
	   // println!("lastmove return: {:?}", lastmovemain);
	    println!("++++++++++++++++++++++++++++++");
    	if lastmovemain == 0{
    		//currentgame.printoutspokes();
    		//println!("All Done!");
    		break;
    	}

    }

}