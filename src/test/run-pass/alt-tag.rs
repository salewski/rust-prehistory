// -*- rust -*-

type color = tag(
  rgb(int, int, int),
  rgba(int, int, int, int),
  hsl(int, int, int)
);

fn process(color c) -> int {
  let int x;
  alt (c) {
    case (rgb(int r, int g, int b)) {
      log "rgb";
      log r;
      x = r;
    }
    case (rgba(int r, int g, int b, int a)) {
      log "rgba";
      log a;
      x = a;
    }
    case (hsl(int h, int s, int l)) {
      log "hsl";
      log s;
      x = s;
    }
  }
  ret x;
}

fn main() {
  let color gray = rgb(127, 127, 127);
  let color clear = rgba(50, 150, 250, 0);
  let color red = hsl(0, 255, 255);
  check (process(gray) == 127);
  check (process(clear) == 0);
  check (process(red) == 255);
}

