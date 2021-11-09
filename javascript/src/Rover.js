export class Rover {

  /** @property {number} #x */
  #x;

  /** @property {number} #y */
  #y;

  /** @property {string} #heading */
  #heading;

  /**
   * @param {number} x
   * @param {number} y
   * @param {string} heading
   */
  constructor(x, y, heading) {
    this.#x = x;
    this.#y = y;
    this.#heading = heading;
  }

  /**
   * @param {string} commands
   * @param {number[][]} obstacles
   * @returns {string}
   */
  execute(commands, obstacles) {
    let stuck = false;

    for (let command of commands.split("")) {
      if (command === "R" && this.#heading === ("N")) {
        this.#heading = "E";
        continue;
      }
      if (command === "R" && this.#heading === "E") {
        this.#heading = "S";
        continue;
      }
      if (command === "R" && this.#heading === "S") {
        this.#heading = "W";
        continue;
      }
      if (command === "R" && this.#heading === "W") {
        this.#heading = "N";
        continue;
      }
      if (command === "L" && this.#heading === "N") {
        this.#heading = "W";
        continue;
      }
      if (command === "L" && this.#heading === "W") {
        this.#heading = "S";
        continue;
      }
      if (command === "L" && this.#heading === "S") {
        this.#heading = "E";
        continue;
      }
      if (command === "L" && this.#heading === "E") {
        this.#heading = "N";
        continue;
      }
      if (command === "M" && this.#heading === "N") {
        let next_y = this.#y + 1;
        if (next_y > 9) {
          next_y = 0;
        }
        if (obstacles.findIndex(elem => elem[0] === this.#x && elem[1] === next_y) > -1) {
          stuck = true;
        }
        if (!stuck) {
          this.#y = next_y;
        }
      }
      if (command === "M" && this.#heading === "S") {
        let next_y = this.#y - 1;
        if (next_y < 0) {
          next_y = 9;
        }
        if (obstacles.findIndex(elem => elem[0] === this.#x && elem[1] === next_y) >= 0) {
          stuck = true;
        }
        if (!stuck) {
          this.#y = next_y;
        }
      }
      if (command === "M" && this.#heading === "E") {
        let next_y = this.#x + 1;
        if (next_y > 9) {
          next_y = 0;
        }
        if (obstacles.findIndex(elem => elem[0] === next_y && elem[1] === this.#y) >= 0) {
          stuck = true;
        }
        if (!stuck) {
          this.#x = next_y;
        }
      }
      if (command === "M" && this.#heading === "W") {
        let next_y = this.#x - 1;
        if (next_y < 0) {
          next_y = 9;
        }

        if (obstacles.findIndex(elem => elem[0] === next_y && elem[1] === this.#y) > -1) {
          stuck = true;
        }
        if (!stuck) {
          this.#x = next_y;
        }
      }
    }

    return (stuck ? "O:" : "") + this.#x + ":" + this.#y + ":" + this.#heading;
  }
}
