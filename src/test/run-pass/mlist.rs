// -*- C -*-

prog a
{
  type visual = tag(shade(int),hue(pix));
  type pix = (colour,colour,colour);
  type colour = tag(red(),green(),blue());
  type mlist = tag(cons(int,@next), nil());
  type next = rec(mlist val);

  main {
    // FIXME: at the moment we can't handle x = conx(10, x)
    // because it'll leak x._1 during the initializing-call.
    // Yuck!
    auto x = nil();
    auto x1 = cons(10, x);
    auto x2 = cons(10, cons(11, cons(12, x1)));
  }
}
