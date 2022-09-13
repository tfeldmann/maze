// Algorithm based on:
// https://weblog.jamisbuck.org/2011/1/27/maze-generation-growing-tree-algorithm

extern crate rand;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
struct Cell {
    x: i32,
    y: i32,
}

const DIR: [i8; 4] = [
    0b0001, // up
    0b0010, // right
    0b0100, // down
    0b1000, // left
];

const OPP_DIR: [i8; 4] = [
    0b0100, // down
    0b1000, // left
    0b0001, // up
    0b0010, // right
];

const DX: [i8; 4] = [0, 1, 0, -1];
const DY: [i8; 4] = [1, 0, -1, 0];

fn maze(width: usize, height: usize) {
    // the grid where we carve the passages between cells
    let mut grid: Vec<Vec<usize>> = vec![vec![0; width]; height];

    // the cells we have to handle
    let mut cells: Vec<Cell> = vec![Cell { x: 0, y: 0 }];

    while !cells.is_empty() {
        // we always start from the last cell
        let cell = &cells[cells.len() - 1];

        // shuffle directions
        let mut dirs: Vec<u8> = (0..4).collect();
        dirs.shuffle(&mut thread_rng());

        for dir in dirs.iter() {
            // let nx: i32 = cell.x + DX.d;
            // let ny: i32 = cell.y + DY.d;
            println!("{:?}", dir);
        }

        println!("{:?}", dirs);
        println!("{:?}", cell);
        println!("{:?}", grid);
        cells.pop();
    }
}

fn main() {
    println!("Maze");
    maze(20, 20);
}

/*
class Maze
{
    // Properties
    // ----------
    int rows, cols;
    int[][] grid;

    int dir[] = {
        unbinary("0001"), // North
        unbinary("0010"), // East
        unbinary("0100"), // South
        unbinary("1000")    // West
    };

    int opposite[] = {
        unbinary("0100"), // South
        unbinary("1000"), // West
        unbinary("0001"), // North
        unbinary("0010")    // East
    };

    int dx[] = {0, 1,    0, -1};
    int dy[] = {1, 0, -1,    0};


    // Constructor and maze generation
    // -------------------------------
    Maze(int cols, int rows)
    {
        // variables
        // ---------
        this.rows = rows;
        this.cols = cols;

        ArrayList cells = new ArrayList();
        grid = new int[cols][rows];


        // Define 0|0 as starting Cell
        // ----------------------------
        cells.add(new MazeCell(0, 0));


        // start growing tree algorithm
        // ----------------------------
        while (cells.size() > 0)
        {

            // backtrack to newest cell
            // ------------------------
            int index = cells.size() - 1;
            MazeCell cell = (MazeCell) cells.get(index);
            int x = cell.x;
            int y = cell.y;


            // shuffle possible directions
            // ---------------------------
            Integer[] directions = new Integer[4];
            for (int i = 0; i < directions.length; i++)
            {
                directions[i] = new Integer(i);
            }
            java.util.Collections.shuffle(java.util.Arrays.asList(directions));


            // try visiting neighbour cells
            // ----------------------------
            for (int i = 0; i < directions.length; i++)
            {
                int selDir = int(directions[i]);
                int nx = x + dx[selDir];
                int ny = y + dy[selDir];


                // if new cell is unvisited carve passage
                // --------------------------------------
                if (nx >= 0 && ny >= 0 && nx < cols && ny < rows && grid[nx][ny] == 0)
                {
                     grid[x][y]     |= dir[selDir];
                     grid[nx][ny] |= opposite[selDir];
                     cells.add(new MazeCell(nx, ny));
                     index = -1;
                     break;
                }
            }

            // dead end: remove cell from list
            // -------------------------------
            if (index != -1)
            {
                cells.remove(index);
            }
        }

    }


    // Drawing Method
    // --------------
    void draw(int cellSize)
    {

        // draw line when no passage is carved
        // -----------------------------------
        for (int y = 0; y < rows; y++)
        {
            for (int x = 0; x < cols; x++)
            {
                // north
                if ((grid[x][y] & dir[0]) == 0)
                    line(x * cellSize, (y+1) * cellSize,
                             (x+1) * cellSize, (y+1) * cellSize);

                // east
                if ((grid[x][y] & dir[1]) == 0)
                    line((x+1) * cellSize, y * cellSize,
                             (x+1) * cellSize, (y+1) * cellSize);

                // south
                if ((grid[x][y] & dir[2]) == 0)
                    line(x * cellSize, y * cellSize,
                             (x+1) * cellSize, y * cellSize);

                // west
                if ((grid[x][y] & dir[3]) == 0)
                    line(x * cellSize, y * cellSize,
                             x * cellSize, (y+1) * cellSize);
            }
        }

    }

}



*/
