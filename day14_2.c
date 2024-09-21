#include <stdio.h>
#include <string.h>

#define LIMIT 100
typedef struct {
  unsigned row;
  char symbol;
} Pos;
typedef struct {
  unsigned count;
  Pos pos[LIMIT];
} Col;
typedef struct {
  unsigned rsize, csize;
  Col cols[LIMIT];
} Board;
Board board;
Board rotated;
void print_board(Board *board) {
  char buffer[LIMIT * LIMIT];
  memset(buffer, '.', LIMIT * LIMIT);

  for (unsigned c = 0; c < board->csize; c += 1) {
    for (unsigned p = 0; p < board->cols[c].count; p += 1) {
      buffer[LIMIT * board->cols[c].pos[p].row + c] = board->cols[c].pos[p].symbol;
    }
  }

  for (unsigned r = 0; r < board->rsize; r += 1) {
    for (unsigned c = 0; c < board->csize; c += 1) {
      printf("%c", buffer[LIMIT * r + c]);
    }
    printf("\n");
  }
}

int main(int argc, char *argv[]) {
    unsigned result = 0;
    char buffer[LIMIT+2];
    unsigned r = 0;
    while (fgets(buffer, LIMIT+2, stdin) != NULL) {
      board.csize = strlen(buffer) - 1;
      buffer[board.csize] = '\0';
      unsigned c = 0;
      for (char *s = buffer; *s; s += 1) {
        if (*s != '.') {
          board.cols[c].pos[board.cols[c].count].row = r;
          board.cols[c].pos[board.cols[c].count].symbol = *s;
          board.cols[c].count += 1;
        }
        c += 1;
      }
      r += 1;
    }
    board.rsize = r;
    rotated.csize = board.rsize;
    rotated.rsize = board.csize;
    for (unsigned c = 0; c < rotated.csize; c += 1) {
      rotated.cols[c].count = 0;
    }
    Board *brd = &board, *rot = &rotated;
    for (unsigned step = 0; step < 4000000000; step += 1) {
      for (unsigned c = 0; c < brd->csize; c += 1) {
        unsigned i = 0;
        for (unsigned p = 0; p < brd->cols[c].count; p += 1) {
          unsigned r = brd->cols[c].pos[p].row;
          char s = brd->cols[c].pos[p].symbol;
          unsigned l = (s == '#') ? r : i;
          unsigned k = brd->rsize - 1 - l;
          unsigned rc = rot->cols[k].count;
          rot->cols[k].pos[rc].row = c;
          rot->cols[k].pos[rc].symbol = s;
          rot->cols[k].count += 1;
          i = l + 1;
        }
        brd->cols[c].count = 0;
      }
      Board *tmp = brd;
      brd = rot;
      rot = tmp;
      if (step%40000 == 0) { printf("%u\n", step/4); }
    }
    printf("\n");
    print_board(brd);
    
    printf("%u", result);
}
