read_file_lines(File, Lines) :- 
    open(File, read, Stream),
    read_lines(Stream, Lines),
    close(Stream).

read_lines(Stream, []) :-
    at_end_of_stream(Stream), !.
read_lines(Stream, [(SpringList, CountsList)|Lines]) :-
    read_line_to_string(Stream, StringLine),
    parse_line(StringLine, SpringList, CountsList),
    read_lines(Stream, Lines).
read_file_line(File, Line) :-
    read_file_lines(File, Lines),
    Lines = [Line|_].
parse_line(Line, SpringsList, CountsList) :-
    split_string(Line, " ", "", List),
    [SpringsString, CountsString] = List,
    parse_springs(SpringsString, SpringsList),
    parse_counts(CountsString, CountsList).
parse_springs(Springs, List) :-
    string_chars(Springs, List).
parse_counts(Counts, CountsList) :-
    split_string(Counts, ",", "", StringList),
    parse_nums(StringList, CountsList).

parse_nums([], []).
parse_nums([StringCount|StringTail], [NumCount|NumTail]) :-
    atom_number(StringCount, NumCount),
    parse_nums(StringTail, NumTail).
copy_springs(0, _, Springs, Springs) :- !.
copy_springs(N, Springs, Copy, Extension) :-
    N1 is N - 1,
    append(Copy, ['?'], S1),
    append(S1, Springs, S2),
    copy_springs(N1, Springs, S2, Extension).
copy_counts(0, _, Counts, Counts) :- !.
copy_counts(N, Counts, Copy, Extension) :-
    N1 is N - 1,
    append(Copy, Counts, C1),
    copy_counts(N1, Counts, C1, Extension).
chunks([], []) :- !.
chunks(['.'|Springs], Chunks) :- chunks(Springs, Chunks), !.
chunks(Springs, [Chunk|Chunks]) :-
    Springs \= [],
    chunk(Springs, [], Chunk, Rest),
    chunks(Rest, Chunks).
chunk([], Chunk, Chunk, []) :- !.
chunk(['.'|Springs], Chunk, Chunk, Springs) :- !.
chunk([Spring|Springs], ChunkIn, [Spring|Chunk], Rest) :-
    Spring \= '.',
    chunk(Springs, ChunkIn, Chunk, Rest).
fixed(Chunk, 0, Chunk) :- !.
fixed(['#'|Chunk], Count, Rest) :-
    Count1 is Count - 1,          %% Тільки для ненульового лічильника відкидаємо перший струмок
    drop(Count1, Chunk, RestOut), %% Відкидаємо з решти послідовність
    separate(RestOut, Rest), !.   %% Відокремлюємо
fixed(['?'|Chunk], Count, Rest) :- fixed(['#'|Chunk], Count, Rest).
fixed(['?'|Chunk], Count, Rest) :- fixed(Chunk, Count, Rest).
separate([], []).
separate(['?'|Rest], Rest).
:- use_module(library(dialect/hprolog)).
:- use_module(library(memo)).
%% :- volatile_memo total(+list).
total([]).
total(['?'|Rest]) :- total(Rest).
%% total([]).
%% total(['?']).
%% total(['?','?']).
%% total(['?','?','?']).
%% total(['?','?','?','?']).
%% total(['?','?','?','?','?']).
%% %%total(comp(unknown,1726236685.5959709,1.9073486328125e-6)-fail,[#,#,?,?,?,?,?,?,?,?,?])).
%% %%total(comp(unknown,1726236685.595964,5.8650970458984375e-5)-fail,[?,#,#,?,?,?,?,?,?,?,?,?])).
%% %%total(comp(unknown,1726236685.5959563,0.00011467933654785156)-fail,[?,?,#,#,?,?,?,?,?,?,?,?,?])).
%% total(['?','?','?','?','?','?']).
%% total(['?','?','?','?','?','?','?']).
%% total(['?','?','?','?','?','?','?','?']).

%% :- volatile_memo possible(+list, +list).
possible([], _).
possible([Chunk|Chunks], Counts) :-
    total(Chunk), !,
    possible(Chunks, Counts).
possible([_|Chunks], [_|Counts]) :- possible(Chunks, Counts).
solution(_, []) :- !. %% possible(Chunks, []), !.
solution([Chunk|Chunks], [Count|Counts]) :-
    %% possible([Chunk|Chunks], [Count|Counts]),
    length(Chunk, L), L >= Count,
    fixed(Chunk, Count, Rest),
    solution([Rest|Chunks], Counts).
solution([Chunk|Chunks], Counts) :-
    total(Chunk),
    solution(Chunks, Counts).
