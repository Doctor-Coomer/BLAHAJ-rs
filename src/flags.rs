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

pub static NONE:   &[(u8, u8, u8)] = &[(  0,   0,   0)];

pub fn get_flag(flag_name: &str) -> &'static [(u8, u8, u8)] {
    if flag_name == "femboy"
    { return FEMBOY; }
    else if flag_name == "trans"
    { return TRANS;  }
    else
    { return NONE;   }
}
