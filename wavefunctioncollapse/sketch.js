
const WIDTH = 400
const HEIGHT = 400
const cellSomething =10

const grid = [] 

for(let i = 0; i < WIDTH/cellSomething; i++){
  grid.push(new Array()) 
  for(let j = 0; j < HEIGHT/cellSomething; j++) {
    grid[i][j] = {
      kind: undefined,
      x: Math.floor(i*cellSomething),
      y: Math.floor(j*cellSomething),
      p: [0,1,2] 
    } 
  }
}
const rows = grid.length
const cols = grid[0].length
function setup() {
  createCanvas(WIDTH,HEIGHT)
  background(200)
  grid[1][1].kind = 0
}

function draw() {
  collapse(grid)
  drawGrid(grid)
  
}
function collapse(grid) {
  for(x in grid) {
    for(y in grid[x]) {

      if( x <= rows && y <= cols) {
             if( x != 0) {
               if(grid[x-1][y+1] == 0) {
               grid[x-1][y].kind = 0 
               } 
             } 
      }
      


        
    }


  }


}

function generateKind() {
  const seed = Math.random(0,1) 
  if(seed > 0.9){
    return 0
  }else if (seed > 0.5) {
    return 1
  }else {
    return 2
  }
}

function drawGrid(grid) {
 for(x of grid)  {
   for(cell of x){
    switch (cell.kind) {
      case 0:
      fill(0)
        break;
      case 1:
       fill(200) 
        break;

      case 2:
       fill(255) 
        break;

      default:
        fill(255)
        break;
    } 
     square(cell.x,cell.y,cellSomething)
   }
 }
}


