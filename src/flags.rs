pub static FEMBOY: &[(u8, u8, u8)] = &[(210,  96, 165),
				       (228, 175, 205),
				       (254, 254, 254),
				       ( 87, 206, 248),
				       (254, 254, 254),
				       (228, 175, 205),
				       (210,  96, 165)];

pub static TRANS:  &[(u8, u8, u8)] = &[( 91, 206, 250),
				       (245, 169, 184),
				       (255, 255, 255),
				       (245, 169, 184),
				       ( 91, 206, 250)];

pub static PRIDE:  &[(u8, u8, u8)] = &[(228,   3,   3),
				       (255, 140,   0),
				       (255, 237,   0),
				       (  0, 128,  38),
				       (  0,  77, 255),
				       (117,   7, 135)];

pub static NONE:   &[(u8, u8, u8)] = &[(  0,   0,   0)];

pub fn get_flag(flag_name: &str) -> &'static [(u8, u8, u8)] {
    if flag_name == "femboy"
    { return FEMBOY; }
    else if flag_name == "trans"
    { return TRANS;  }
    else if flag_name == "pride"
    { return PRIDE;  }
    else
    { return NONE;   }
}
