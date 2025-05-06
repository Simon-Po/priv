import './index.css'

enum State {
  q0 = "q0",
  q_halt = "q_halt"
}
const bttn = document.querySelector('#button') as HTMLButtonElement

class TouringMachine {
  position: number = 0
  band
  state: State
  constructor() {
    this.band = ["0", "0", "1", "1", "0", "1", "1", "0", "1", "0", "B"]
    this.state = State.q0
  }

  public debug_print(): void {
    console.log(this.band.map(i => `[${i}]`).join(""))
    console.log(" ".repeat(this.position * 3), "^", " ".repeat((this.band.length - this.position) * 3))
  }

  runNextCommand(): boolean {
    switch (this.state){
        case State.q0:
          if(this.band[this.position] === "B") {
            this.state = State.q_halt
            return true;
          }else {
          this.band[this.position] = this.band[this.position] === "0" ? "1" : "0";
          this.position++
          this.state = State.q0
          return true
          }
        case State.q_halt:
          return false
    }
   
  }

}

function setup() {
  const canvas = document.querySelector('#machine') as HTMLCanvasElement | null
  if (!canvas) {
    throw new Error('Canvas element with id "machine" not found.')
  }
  const ctx = canvas.getContext('2d')
  if (!ctx) {
    throw new Error('2D context not available.')
  }
  return { canvas, ctx }
}

function drawTM(canvas: HTMLCanvasElement, ctx: CanvasRenderingContext2D, t: TouringMachine) {
  // Draw something simple
  ctx.fillStyle = 'black'
  // clear background
  ctx.fillRect(0, 0, canvas.width, canvas.height)


  ctx.strokeStyle = 'white'
  ctx.strokeRect(10, 10, 200, 200)
}

function main() {
  const { canvas, ctx } = setup()

  const t = new TouringMachine()
  t.debug_print()

  // Set canvas size
  canvas.width = 800
  canvas.height = 400
  drawTM(canvas, ctx, t)
  bttn.onclick = () => { 
    const result = t.runNextCommand()
    if(result) {
      t.debug_print() 
    } else {
      console.log("All done")
    }
  }
}


main()
