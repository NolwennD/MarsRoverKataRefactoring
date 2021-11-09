import { Rover } from '@/Rover';

describe('Rover', () => {
  it('Should do two 360', () => {
    const rover = new Rover(0, 0, "N");

    const res = rover.execute("RRRRLLLL", []);

    expect(res).toBe("0:0:N");
  });

  it('Should wrap at South and North', () => {
    const rover = new Rover(0, 0, "S");

    const res = rover.execute("MLLM", []);

    expect(res).toBe("0:0:N");
  });

  it('Should wrap at East and West', () => {
    const rover = new Rover(0, 0, "W");

    const res = rover.execute("MLLM", []);

    expect(res).toBe("0:0:E");
  });

  it('Should hit an obstacle on North', () => {
    const rover = new Rover(0, 0, "N");

    const res = rover.execute("M", [[0, 1]]);

    expect(res).toBe("O:0:0:N");
  });

  it('Should hit an obstacle on South', () => {
    const rover = new Rover(0, 1, "S");

    const res = rover.execute("M", [[0, 0]]);

    expect(res).toBe("O:0:1:S");
  });

  it('Should hit an obstacle on west', () => {
    const rover = new Rover(1, 0, "W");

    const res = rover.execute("M", [[0, 0]]);

    expect(res).toBe("O:1:0:W");
  });

  it('Should hit an obstacle on East', () => {
    const rover = new Rover(0, 0, "E");

    const res = rover.execute("M", [[1, 0]]);

    expect(res).toBe("O:0:0:E");
  });
});
