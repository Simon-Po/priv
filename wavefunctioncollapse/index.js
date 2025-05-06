
const WIDTH = 400
const HEIGHT = 400
const cellSomething =10 

const grid = [] 

for(let i = 0; i < WIDTH/cellSomething; i++){
  grid.push(new Array()) 
  for(let j = 0; j < HEIGHT/cellSomething; j++) {
    grid[i][j] = 0 
  }
}

function setup() {
  createCanvas(WIDTH,HEIGHT)
  background(200)
  console.table(grid)
}

function draw() {
  ellipse(200,200,50,50)


}




