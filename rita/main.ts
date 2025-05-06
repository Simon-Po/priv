abstract class Shape{
  private a: number

  constructor() {

  }
  area() {
    console.log("area not implemented for this shape")
  }

}


class Square {
  
  private a: number

  constructor(a: number) {
    this.a = a;
    console.log("hey Rita")
  }
  
  area(): number {
    return this.a*this.a;
  }
}
class EqualTriangle extends Shape {
  constructor(a: number) {
    super()
  }
}




function main() {
  const mySquare = new Square(2)
  const e = new EqualTriangle(4)

  console.log(mySquare.area())
  console.log(e.area())
}

main()
