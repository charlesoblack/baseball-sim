#! /usr/bin/env python


class Player(object):

    def __init__(self):
        self.init_stats()
        self.init_tools()

    def init_tools(self):
        import random

        self.power = random.randint(0, 100)
        self.eye = random.randint(0, 100)
        self.speed = random.randint(0, 100)
        self.instinct = random.randint(0, 100)
        self.coordination = random.randint(0, 100)

    def init_stats(self):
        self.pas = 0
        self.walks = 0
        self.singles = 0
        self.doubles = 0
        self.triples = 0
        self.hrs = 0
        self.rbis = 0
        self.hbps = 0
        self.sac_flies = 0

    @property
    def abs(self):
        # not entirely correct
        return self.pas - self.walks

    @property
    def ba(self):
        return self.hits / self.abs

    @property
    def hits(self):
        return self.singles + self.doubles + self.triples + self.hrs

    @property
    def obp(self):
        return (self.hits + self.walks + self.hbps) / (self.abs + self.walks + self.hbps + self.sac_flies)

    @property
    def ops(self):
        return self.obp + self.slg

    def plate_appearance(self):
        import random

        self.pas += 1

        chance = random.randint(0, 100)

        if chance < self.coordination:
            hit = random.randint(0, 100)
            if hit > self.power:
                self.doubles += 1
            else:
                self.singles += 1
            print("got a hit! batting average {:.3f}".format(self.ba, flush=True))


example_player = Player()

for i in range(0, 100):
    example_player.plate_appearance()

