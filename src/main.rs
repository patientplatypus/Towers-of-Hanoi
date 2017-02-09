

use std::io;


extern crate rand;

use rand::distributions::{IndependentSample, Range};


fn computercontrol(mut gametable: Vec<Vec<&str>>) -> Vec<Vec<&str>> {

  //  println!("running computer control");


	let mut possiblegametable: Vec<Vec<&str>> = gametable;





    //let mut possiblemovevec = String::new();


    //ACHTUNG ACHTUNG

    //Have to add test to make sure that the computer doesn't decide to go on a column that is already filled!

  
    //check if computer can win in any possible move for itself given the current game board

	
    if possiblegametable[1][1] == "." {

    //	println!("check1");
   // 	println!("{:?}", possiblegametable);

		let mut possiblewhosturn = String::new();
		possiblewhosturn = "computer".to_string();
		let mut possiblemovevec = String::new();
		let mut x = "a";
		possiblemovevec = x.to_string();
		updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
		let mut possiblewinnerstring = check4winner(&mut possiblegametable);

		println!("{:?}", possiblewinnerstring);

		if possiblewinnerstring != "no winners yet desu~"{

			return possiblegametable;
		} else{

			resetgametable(&mut possiblegametable, &mut possiblemovevec);

		}

	}

	if possiblegametable[1][2] == "." {

	//	println!("check2");
   // 	println!("{:?}", possiblegametable);

		let mut possiblewhosturn = String::new();
		possiblewhosturn = "computer".to_string();
		let mut possiblemovevec = String::new();
		let mut x = "b";
		possiblemovevec = x.to_string();
		updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
		let mut possiblewinnerstring = check4winner(&mut possiblegametable);	

		if possiblewinnerstring != "no winners yet desu~"{
			return possiblegametable;
		}else{

			resetgametable(&mut possiblegametable, &mut possiblemovevec);

		}

	
	}

	if possiblegametable[1][3]=="." {

		let mut possiblewhosturn = String::new();
		possiblewhosturn = "computer".to_string();
		let mut possiblemovevec = String::new();
		let mut x = "c";
		possiblemovevec = x.to_string();
		updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
		let mut possiblewinnerstring = check4winner(&mut possiblegametable);


		if possiblewinnerstring != "no winners yet desu~"{
			return possiblegametable;	
		}else{

			resetgametable(&mut possiblegametable, &mut possiblemovevec);

		}

	}


	if possiblegametable[1][4]=="." {

		let mut possiblewhosturn = String::new();
		possiblewhosturn = "computer".to_string();
		let mut possiblemovevec = String::new();
		let mut x = "d";
		possiblemovevec = x.to_string();
		updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
		let mut possiblewinnerstring = check4winner(&mut possiblegametable);


		if possiblewinnerstring != "no winners yet desu~"{
			return possiblegametable;
		}else{

			resetgametable(&mut possiblegametable, &mut possiblemovevec);

		}

	}

	if possiblegametable[1][5]=="." {

		let mut possiblewhosturn = String::new();
		possiblewhosturn = "computer".to_string();
		let mut possiblemovevec = String::new();
		let mut x = "e";
		possiblemovevec = x.to_string();
		updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
		let mut possiblewinnerstring = check4winner(&mut possiblegametable);


		if possiblewinnerstring != "no winners yet desu~"{
			return possiblegametable;
		}else{

			resetgametable(&mut possiblegametable, &mut possiblemovevec);

		}

	}

	if possiblegametable[1][6]=="." {

		let mut possiblewhosturn = String::new();
		possiblewhosturn = "computer".to_string();
		let mut possiblemovevec = String::new();
		let mut x = "f";
		possiblemovevec = x.to_string();
		updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
		let mut possiblewinnerstring = check4winner(&mut possiblegametable);


		if possiblewinnerstring != "no winners yet desu~"{
			return possiblegametable;	
		}else{

			resetgametable(&mut possiblegametable, &mut possiblemovevec);

		}

	}

	if possiblegametable[1][7]=="." {

		let mut possiblewhosturn = String::new();
		possiblewhosturn = "computer".to_string();
		let mut possiblemovevec = String::new();
		let mut x = "g";
		possiblemovevec = x.to_string();
		updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
		let mut possiblewinnerstring = check4winner(&mut possiblegametable);


		if possiblewinnerstring != "no winners yet desu~"{
			return possiblegametable;
		}else{

			resetgametable(&mut possiblegametable, &mut possiblemovevec);

		}

	}



    //OTHERWISE
    //check if player can win in all possible hypothetical moves given that computer skips its turn
    //if the player can win there then the computer will go there instead
  	
  	
    if possiblegametable[1][1] == "." {

		let mut possiblewhosturn = String::new();
		possiblewhosturn = "player".to_string();
		let mut possiblemovevec = String::new();
		let mut x = "a";
		possiblemovevec = x.to_string();
		updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
		let mut possiblewinnerstring = check4winner(&mut possiblegametable);

		if possiblewinnerstring != "no winners yet desu~"{
			resetgametable(&mut possiblegametable, &mut possiblemovevec);
			possiblewhosturn = "computer".to_string();
			x = "a";
			possiblemovevec = x.to_string();
			updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
			return possiblegametable;
		}else{

			resetgametable(&mut possiblegametable, &mut possiblemovevec);

		}

	}

	if possiblegametable[1][2] == "." {

		let mut possiblewhosturn = String::new();
		possiblewhosturn = "player".to_string();
		let mut possiblemovevec = String::new();
		let mut x = "b";
		possiblemovevec = x.to_string();
		updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
		let mut possiblewinnerstring = check4winner(&mut possiblegametable);	

		if possiblewinnerstring != "no winners yet desu~"{
			resetgametable(&mut possiblegametable, &mut possiblemovevec);
			possiblewhosturn = "computer".to_string();
			x = "b";
			possiblemovevec = x.to_string();
			updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
			return possiblegametable;
		}else{

			resetgametable(&mut possiblegametable, &mut possiblemovevec);

		}

	}

	if possiblegametable[1][3]=="." {

		let mut possiblewhosturn = String::new();
		possiblewhosturn = "player".to_string();
		let mut possiblemovevec = String::new();
		let mut x = "c";
		possiblemovevec = x.to_string();
		updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
		let mut possiblewinnerstring = check4winner(&mut possiblegametable);


		if possiblewinnerstring != "no winners yet desu~"{
			resetgametable(&mut possiblegametable, &mut possiblemovevec);
			possiblewhosturn = "computer".to_string();
			x = "c";
			possiblemovevec = x.to_string();
			updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
			return possiblegametable;
		}else{

			resetgametable(&mut possiblegametable, &mut possiblemovevec);

		}

	}


	if possiblegametable[1][4]=="." {

		let mut possiblewhosturn = String::new();
		possiblewhosturn = "player".to_string();
		let mut possiblemovevec = String::new();
		let mut x = "d";
		possiblemovevec = x.to_string();
		updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
		let mut possiblewinnerstring = check4winner(&mut possiblegametable);


		if possiblewinnerstring != "no winners yet desu~"{
			resetgametable(&mut possiblegametable, &mut possiblemovevec);
			possiblewhosturn = "computer".to_string();
			x = "d";
			possiblemovevec = x.to_string();
			updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
			return possiblegametable;
		}else{

			resetgametable(&mut possiblegametable, &mut possiblemovevec);

		}

	}

	if possiblegametable[1][5]=="."{

		let mut possiblewhosturn = String::new();
		possiblewhosturn = "player".to_string();
		let mut possiblemovevec = String::new();
		let mut x = "e";
		possiblemovevec = x.to_string();
		updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
		let mut possiblewinnerstring = check4winner(&mut possiblegametable);


		if possiblewinnerstring != "no winners yet desu~"{
			resetgametable(&mut possiblegametable, &mut possiblemovevec);
			possiblewhosturn = "computer".to_string();
			x = "e";
			possiblemovevec = x.to_string();
			updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
			return possiblegametable;
		}else{

			resetgametable(&mut possiblegametable, &mut possiblemovevec);

		}

	}

	if possiblegametable[1][6]=="." {

		let mut possiblewhosturn = String::new();
		possiblewhosturn = "player".to_string();
		let mut possiblemovevec = String::new();
		let mut x = "f";
		possiblemovevec = x.to_string();
		updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
		let mut possiblewinnerstring = check4winner(&mut possiblegametable);


		if possiblewinnerstring != "no winners yet desu~"{
			resetgametable(&mut possiblegametable, &mut possiblemovevec);
			possiblewhosturn = "computer".to_string();
			x = "f";
			possiblemovevec = x.to_string();
			updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
			return possiblegametable;
		}else{

			resetgametable(&mut possiblegametable, &mut possiblemovevec);

		}

	}

	if possiblegametable[1][7]=="." {

		let mut possiblewhosturn = String::new();
		possiblewhosturn = "player".to_string();
		let mut possiblemovevec = String::new();
		let mut x = "g";
		possiblemovevec = x.to_string();
		updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
		let mut possiblewinnerstring = check4winner(&mut possiblegametable);


		if possiblewinnerstring != "no winners yet desu~"{
			resetgametable(&mut possiblegametable, &mut possiblemovevec);
			possiblewhosturn = "computer".to_string();
			x = "g";
			possiblemovevec = x.to_string();
			updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
			return possiblegametable;
		}else{

			resetgametable(&mut possiblegametable, &mut possiblemovevec);

		}

	}


    //OTHERWISE OTHERWISE
    //the computer will simply decide to go somewhere at random

		let mut possiblewhosturn = String::new();
		possiblewhosturn = "computer".to_string();

		let mut rng = rand::thread_rng();
	    let mut between = Range::new(1, 8);
	    let mut pickslot = between.ind_sample(&mut rng);


		loop{
	    	rng = rand::thread_rng();
	    	between = Range::new(1, 8);
	    	pickslot = between.ind_sample(&mut rng);

	    	if possiblegametable[1][pickslot] == "."{
	    		break;
	    	}

		}

		if pickslot == 1 {

			let mut x = "a";
			let mut possiblemovevec = String::new();
			let mut possiblemovevec = x.to_string();
			updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
			return possiblegametable;
		

		} else if pickslot == 2 {
			
			let mut x = "b";
			let mut possiblemovevec = String::new();
			let mut possiblemovevec = x.to_string();
			updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
			return possiblegametable;
		

		} else if pickslot == 3 {
			
			let mut x = "c";
			let mut possiblemovevec = String::new();
			let mut possiblemovevec = x.to_string();
			updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
			return possiblegametable;


		} else if pickslot == 4 {

			let mut x = "d";
			let mut possiblemovevec = String::new();
			let mut possiblemovevec = x.to_string();
			updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
			return possiblegametable;

		
		} else if pickslot == 5 {
			
			let mut x = "e";
			let mut possiblemovevec = String::new();
			let mut possiblemovevec = x.to_string();
			updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
			return possiblegametable;


		} else if pickslot == 6 {
			
			let mut x = "f";
			let mut possiblemovevec = String::new();
			let mut possiblemovevec = x.to_string();
			updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
			return possiblegametable;

		
		} else if pickslot == 7 {
			
			let mut x = "g";
			let mut possiblemovevec = String::new();
			let mut possiblemovevec = x.to_string();
			updategametable(&mut possiblegametable, &mut possiblemovevec, &mut possiblewhosturn);
			return possiblegametable;

		} else {
			return possiblegametable;
		}



		

}


fn check4winner(gametable: &mut Vec<Vec<&str>>) -> String{


	//let mut returnwinner = String::new();

	let mut returnwinner = "".to_string();

	'loopthroughrows: for x in 1..7{
			'loopthroughcolumns: for y in 1..8{
				//let mut stringtotest = String::new();
				let mut stringtotest = gametable[x][y].to_string();

				if stringtotest != "."{

					//test columns first
					if y+1<=7 && y+2<=7 && y+3<=7{
						//println!("1");
						//println!("x {:?}", x);
						//println!("y {:?}", y);
						//println!("stringtotest {:?}", stringtotest);
						//println!("gametable[x][y]: {:?}", gametable[x][y]);
						//println!("gametable[x][y+1]: {:?}", gametable[x][y+1]);
						//println!("gametable[x][y+2]: {:?}", gametable[x][y+2]);
						//println!("gametable[x][y+3]: {:?}", gametable[x][y+3]);						
						//println!("x {:?}", x);
						if stringtotest == gametable[x][y+1] && stringtotest == gametable[x][y+2] && stringtotest == gametable[x][y+3]{
							//println!("Yolo");
							returnwinner = stringtotest.to_string();
							break 'loopthroughrows;
						}
					}

					if y>=2 && y+1<=7 && y+2<=7{
						//println!("2");
						if stringtotest == gametable[x][y-1] && stringtotest == gametable[x][y+1] && stringtotest == gametable[x][y+2]{
							returnwinner = stringtotest.to_string();
							break 'loopthroughrows;
						}
					}

					if y>=3 && y>=2 && y+1<=7{
						//println!("3");
						if stringtotest == gametable[x][y-2] && stringtotest == gametable[x][y-1] && stringtotest == gametable[x][y+1]{
							returnwinner = stringtotest.to_string();
							break 'loopthroughrows;
						}
					}


					if y>=4 && y>=3 && y>=2{
						//println!("4");
						if stringtotest == gametable[x][y-3] && stringtotest == gametable[x][y-2] && stringtotest == gametable[x][y-1]{
							returnwinner = stringtotest.to_string();
							break 'loopthroughrows;
						}
					}

					//then test rows

					if x+1<=6 && x+2<=6 && x+3<=6{
						//println!("5");
						if stringtotest == gametable[x+1][y] && stringtotest == gametable[x+2][y] && stringtotest == gametable[x+3][y]{
							returnwinner = stringtotest.to_string();
							break 'loopthroughrows;
						}
					}

					if x>=2 && x+1<=6 && x+2<=6{
						//println!("6");
						if stringtotest == gametable[x-1][y] && stringtotest == gametable[x+1][y] && stringtotest == gametable[x+2][y]{
							returnwinner = stringtotest.to_string();
							break 'loopthroughrows;
						}
					}

					if x>=3 && x>=2 && x+1<=6{
						//println!("7");
						if stringtotest == gametable[x-2][y] && stringtotest == gametable[x-1][y] && stringtotest == gametable[x+1][y]{
							returnwinner = stringtotest.to_string();
							break 'loopthroughrows;
						}
					}


					if x>=4 && x>=3 && x>=2{
						//println!("8");
						if stringtotest == gametable[x-3][y] && stringtotest == gametable[x-2][y] && stringtotest == gametable[x-1][y]{
							returnwinner = stringtotest.to_string();
							break 'loopthroughrows;
						}
					}

					//then test diagonals

					if y+1<=7 && y+2<=7 && y+3<=7 && x+1<=6 && x+2<=6 && x+3<=6{
						//println!("9");
						if stringtotest == gametable[x+1][y+1] && stringtotest == gametable[x+2][y+2] && stringtotest == gametable[x+3][y+3]{
							returnwinner = stringtotest.to_string();
							break 'loopthroughrows;
						}
					}

					if y>=2 && y+1<=7 && y+2<=7 && x-1>=1 && x+1<=6 && x+2<=6{
						//println!("10");
						if stringtotest == gametable[x-1][y-1] && stringtotest == gametable[x+1][y+1] && stringtotest == gametable[x+2][y+2]{
							returnwinner = stringtotest.to_string();
							break 'loopthroughrows;
						}
					}

					if y>=3 && y>=2 && y+1<=7 && x>=3 && x>=2 && x+1<=6{
						//println!("11");
						if stringtotest == gametable[x-2][y-2] && stringtotest == gametable[x-1][y-1] && stringtotest == gametable[x+1][y+1]{
							returnwinner = stringtotest.to_string();
							break 'loopthroughrows;
						}
					}


					if y>=4 && y>=3 && y>=2 && x>=4 && x>=3 && x>=2{
						//println!("12");
						if stringtotest == gametable[x-3][y-3] && stringtotest == gametable[x-2][y-2] && stringtotest == gametable[x-1][y-1]{
							returnwinner = stringtotest.to_string();
							break 'loopthroughrows;
						}
					}

				}	




			}
	}

	let mut tiecounter = 0;


	'loopthroughrows: for x in 1..7{
			'loopthroughcolumns: for y in 1..8{

				let mut stringtotest = gametable[x][y].to_string();

				if stringtotest != "."{
					tiecounter += 1;
				}
			}
	}

	if tiecounter == 0{

		returnwinner = "We ended in a tie!".to_string();

	}





	if returnwinner == ""{
		returnwinner = "no winners yet desu~".to_string();
	}

	return returnwinner;

}




fn resetgametable(gametable: &mut Vec<Vec<&str>>, movevec: &mut String){


	let mut localmovevalue = String::new();

	let mut localmovevalue = movevec;

	//println!("Here is a localmove variable: {:?}", localmovevalue);

   	//let mut xsandos = String::new();

	//if whosturn == "player"{
	//	println!("inside the if");
    //    xsandos = "player".to_string();
	//}else if whosturn == "computer"{
	//	println!("inside the if");
    //    xsandos = "computer".to_string();
	//}

	let mut xsandos = ".";


	//println!("whosturn {:?}", whosturn);
	//println!("XsOs {:?}", xsandos);

	if localmovevalue == "a\n" || localmovevalue == "a"{
		'xloopera: for x in 1..7{
			if gametable[x][1] == "X" || gametable[x][1] == "O"{
				gametable[x][1] = xsandos;
				break 'xloopera;
			}
		}
	} else if localmovevalue == "b\n" || localmovevalue == "b"{
		'xlooperb: for x in 1..7{
			if gametable[x][2] == "X" || gametable[x][2] == "O"{
				gametable[x][2] = xsandos;
				break 'xlooperb;
			}
		}
	} else if localmovevalue == "c\n" || localmovevalue == "c"{
		'xlooperc: for x in 1..7{
			if gametable[x][3] == "X" || gametable[x][3] == "O"{
				gametable[x][3] = xsandos;
				break 'xlooperc;
			}
		}
	} else if localmovevalue == "d\n" || localmovevalue == "d"{
		'xlooperd: for x in 1..7{
			if gametable[x][4] == "X" || gametable[x][4] == "O"{
				gametable[x][4] = xsandos;
				break 'xlooperd;
			}
		}
	} else if localmovevalue == "e\n" || localmovevalue == "e"{
		'xloopere: for x in 1..7{
			if gametable[x][5] == "X" || gametable[x][5] == "O"{
				gametable[x][5] = xsandos;
				break 'xloopere;
			}
		}
	} else if localmovevalue == "f\n" || localmovevalue == "f"{
		'xlooperf: for x in 1..7{
			if gametable[x][6] == "X" || gametable[x][6] == "O"{
				gametable[x][6] = xsandos;
				break 'xlooperf;
			}
		}
	} else if localmovevalue == "g\n" || localmovevalue == "g"{
		'xlooperg: for x in 1..7{
			if gametable[x][7] == "X" || gametable[x][7] == "O"{
				gametable[x][7] = xsandos;
				break 'xlooperg;
			}
		}
	}






}




fn updategametable(gametable: &mut Vec<Vec<&str>>, movevec: &mut String, whosturn: &mut String){

	//println!("Here is the game table: {:?}", gametable);

	let mut localmovevalue = String::new();

	let mut localmovevalue = movevec;

	//println!("Here is a localmove variable: {:?}", localmovevalue);

   	//let mut xsandos = String::new();

	//if whosturn == "player"{
	//	println!("inside the if");
    //    xsandos = "player".to_string();
	//}else if whosturn == "computer"{
	//	println!("inside the if");
    //    xsandos = "computer".to_string();
	//}

	let mut xsandos = "";

	xsandos = if whosturn=="player"{"O"} else if whosturn=="computer"{"X"} else {"We got an error!"};

	//println!("whosturn {:?}", whosturn);
	//println!("XsOs {:?}", xsandos);

	if localmovevalue == "a\n" || localmovevalue == "a"{
		'xloopera: for x in 1..7{
			if gametable[7-x][1] == "."{
				gametable[7-x][1] = xsandos;
				break 'xloopera;
			}
		}
	} else if localmovevalue == "b\n" || localmovevalue == "b"{
		'xlooperb: for x in 1..7{
			if gametable[7-x][2] == "."{
				gametable[7-x][2] = xsandos;
				break 'xlooperb;
			}
		}
	} else if localmovevalue == "c\n" || localmovevalue == "c"{
		'xlooperc: for x in 1..7{
			if gametable[7-x][3] == "."{
				gametable[7-x][3] = xsandos;
				break 'xlooperc;
			}
		}
	} else if localmovevalue == "d\n" || localmovevalue == "d"{
		'xlooperd: for x in 1..7{
			if gametable[7-x][4] == "."{
				gametable[7-x][4] = xsandos;
				break 'xlooperd;
			}
		}
	} else if localmovevalue == "e\n" || localmovevalue == "e"{
		'xloopere: for x in 1..7{
			if gametable[7-x][5] == "."{
				gametable[7-x][5] = xsandos;
				break 'xloopere;
			}
		}
	} else if localmovevalue == "f\n" || localmovevalue == "f"{
		'xlooperf: for x in 1..7{
			if gametable[7-x][6] == "."{
				gametable[7-x][6] = xsandos;
				break 'xlooperf;
			}
		}
	} else if localmovevalue == "g\n" || localmovevalue == "g"{
		'xlooperg: for x in 1..7{
			if gametable[7-x][7] == "."{
				gametable[7-x][7] = xsandos;
				break 'xlooperg;
			}
		}
	}



}








//fn updatemovevec<'a>(movevec: &mut Vec<&'a str>, mut input: &'a mut String) {
//fn updatemovevec(movevec: &mut Vec<&str>, mut input: &mut String) {
fn getinput(mut input: &mut String) {

    loop {
		input.clear();
		//let mut input = String::new();
    	io::stdin().read_line(&mut input)
        	.ok()
        	.expect("Couldn't read line"); 


     //   println!("{:?}", input);

        if input == "a\n" || input == "b\n" || input == "c\n" || input == "d\n" || input == "e\n" || input== "f\n" || input == "g\n" {
        	break;
        } else {
        	println!("Please only input letters from a through g!");
        	continue;
        }
    }

  //  println!("hello {}", input);
    
}


fn main() {

	//initial game print

    println!("Hello, and welcome!");
    println!("We are going to play connect4");
    println!("Here is the game grid:");
    println!("");
    println!("    a b c d e f g");
    println!("6   . . . . . . . ");
    println!("5   . . . . . . . ");
    println!("4   . . . . . . . ");
    println!("3   . . . . . . . ");
    println!("2   . . . . . . . ");
    println!("1   . . . . . . . ");
    println!("");
    println!("In order to play print a lower case letter corresponding to the column you would like to drop you piece down (ie 'a')");
    println!("Your pieces will be represented by O's and the computer's pieces will be represented by X's");
    //println!("You go first! Enter your first move:");



		   let mut gametable: Vec<Vec<&str>> = vec![vec![" ","a","b","c","d","e","f","g"], vec!["6",".",".",".",".",".",".","."],
		                                        vec!["5",".",".",".",".",".",".","."], vec!["4",".",".",".",".",".",".","."],                                                         
		                                        vec!["3",".",".",".",".",".",".","."], vec!["2",".",".",".",".",".",".","."], 
		                                        vec!["1",".",".",".",".",".",".","."]];


    loop{

    	   println!("Please enter your move:");


		   let mut whosturn = String::new();
		   whosturn = "player".to_string();
		  // println!("{}",whosturn);


		   let mut input = String::new();
		   let mut movevec = String::new();
		   getinput(&mut input);
		   movevec = input;
		   
		   updategametable(&mut gametable, &mut movevec, &mut whosturn);

		   println!("Here is the game table after the player's turn: ");



			'loopthroughrows: for x in 0..7{
				'loopthroughcolumns: for y in 0..8{
					
					print!("{} ", gametable[x][y]);

				}

				println!("");

			}


		   let mut winnerstring = check4winner(&mut gametable);

		   if winnerstring == "We ended in a tie!"{

		   		println!("We ended in a tie!");

		   }

		   if winnerstring != "no winners yet desu~"{

		   		println!("Player wins!");
		   		break;
		   }


		   println!("Now it's the computer's turn!");


		   let mut whosturn = String::new();
		   whosturn = "computer".to_string();
		//   println!("{}",whosturn);


		   gametable = computercontrol(gametable);		   



		   println!("Here is the game table after the computer's turn: ");



			'loopthroughrows: for x in 0..7{
				'loopthroughcolumns: for y in 0..8{
					
					print!("{} ", gametable[x][y]);

				}

				println!("");

			}



		   let mut winnerstring = check4winner(&mut gametable);


		   if winnerstring == "We ended in a tie!"{

		   		println!("We ended in a tie!");

		   }


		   if winnerstring != "no winners yet desu~"{

		   		println!("Computer wins!");
		   		break;
		   }


		   println!("next turn");






		}




}


//println!("movevecValue {:?}", movevec);

//trial

 //  updatemovevec(&mut movevec, &mut input);
 //  whosturn = "player".to_string();
 //  updategametable(&mut gametable, &mut movevec, &mut whosturn);


   //order of declaration of input and movevec matters because updatemovevec 
   //destructs variables in the reverse order that they are called in

  
  // let mut input = "";
  // let mut movevec: Vec<&str> = Vec::new();
  //	 let mut movevec: Vec<&str> = Vec::new();
 //  let mut movevec_copy = movevec.clone();
  //movevec.push(input);
  //  println!("movevecCLONEValue {:?}", movevec_copy);

	//the following commented out code compiles correctly

	// println!("length of gametable {}", gametable.len());
	//for x in 0..gametable.len(){
	//	println!("{:?}", gametable[x])
	//}