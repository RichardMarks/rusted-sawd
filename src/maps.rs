// ###############################    ^          ^       ^    ##  ^   ^    ^    T  ^  H ^   ##WWWW          T          ^  ## WWWW  ^      .........     ##  T WWWW T    .       ..    ##     WWWWW    .        ..   ##     T WW......         ..  ##.....   W.WWWWW    T     ...##    ......WWWWWWW WWWWWWWW  ## T       .  WWWWWWWW   WWWWW##   H T   .  T  WWWW     WWWW##         .   T   WWWWWWWWWWW###############################

use std::collections::HashSet;

pub static WORLD_MAP: &str = "###############################    ^          ^       ^    ##  ^   ^    ^    T  ^  H ^   ##WWWW          T          ^  ## WWWW  ^      .........     ##  T WWWW T    .       ..    ##     WWWWW    .        ..   ##     T WW......         ..  ##.....   W.WWWWW    T     ...##    ......WWWWWWW WWWWWWWW  ## T       .  WWWWWWWW   WWWWW##   H T   .  T  WWWW     WWWW##         .   T   WWWWWWWWWWW###############################";

pub static KIRAR_SOUTH_MAP: &str = "#########################.#####                ^   T  #.#  ##    ^  ^      T        ..   ##   ^      H ^      ^ ...    ##     H    ... ^      . ^    ##    ^.  ^  T...     ...H    ##     ....   ^ . ^....   ^   ##    ^   .     ....          ##      H...H    ^...H     T  ## ^    ^ . ^ .....   ^  ^    ##   T   ......  ^            ##  ^    .   ^       T    ^   ##      #.#    ^       ^^     #########.#####################";

pub static KIRAR_NORTH_MAP: &str = "###############################WWWWWWWWWWWWWWWWWWWWWWWWWWWW##WW########################WW##WW#TT^TTTTTT   ^   ..H  ^#WW##WW#TTH..TT^     ^  .  ^  #WW##WW#TTT ...  ^     ^.     #WW##WW#  TT^ ...    T  .^    #WW##WW#      ^ ..    ...    ^#WW##WW# ^     T . ^  .       #WW##WW#    H..........  T ^  #WW##WW# ^   ^     .          #WW##WW############.###########WW##WWWWWWWWWWWWW#.#WWWWWWWWWWWW################.##############";

pub static DUSHAL_WEST_MAP: &str = "###############################                            ##         T     T     T      ##      T                     ##            T  H  T     T   ##   T         ...           ###             .       ........#       H...  .........     ### T         ....             ##   T     T          T   T   ##             T              ##      T                T    ##            T   T           ###############################";

pub static DUSHAL_EAST_MAP: &str = "###############################                            ###    T   H   T          H   #....      ...    ...  T  .   ### ...    T  ..... .......   ##    ...      ..  T   ..     ##      ..   ...        .     ##     .......          . T   ##  T ..  T       T     .     ##   ..            ........  ###  .. T           H      .....#  HH        T              ###                     T      ###############################";

pub const MAP_COUNT: usize = 5;

pub static MAPS: [&str; MAP_COUNT] = [
    WORLD_MAP,
    KIRAR_NORTH_MAP,
    KIRAR_SOUTH_MAP,
    DUSHAL_WEST_MAP,
    DUSHAL_EAST_MAP,
];

pub const WORLD_MAP_ID: usize = 0;
pub const KIRAR_NORTH_MAP_ID: usize = 1;
pub const KIRAR_SOUTH_MAP_ID: usize = 2;
pub const DUSHAL_WEST_MAP_ID: usize = 3;
pub const DUSHAL_EAST_MAP_ID: usize = 4;

pub const MAP_W: i32 = 30;
pub const MAP_H: i32 = 14;

pub fn is_cell_empty(x: i32, y: i32, map: &Vec<char>) -> bool {
    let x: i32 = x.clamp(0, MAP_W - 1);
    let y: i32 = y.clamp(0, MAP_H - 1);
    let tile: char = map[(x + (y * MAP_W)) as usize];

    let walkable_tiles: HashSet<char> = HashSet::from([' ', '.', 'H']);
    walkable_tiles.contains(&tile)
}
