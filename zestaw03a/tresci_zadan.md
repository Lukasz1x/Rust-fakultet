# Zestaw 3a
1. Napisz funkcję dwuargumentową, która zamieni wartości podanych w argumentach zmiennych (dla ustalenia uwagi: typu i32).
2. Napisz funkcję trójargumentową, która poprzestawia wartości swoich argumentów (dla ustalenia uwagi: typu i32) tak, by były uporządkowane niemalejąco.
3. Stwórz generator liczb pseudolosowych, którego ziarno przechowywane będzie na zewnątrz i podawane w pierwszym parametrze, a w parametrze drugim i trzecim podawany będzie przedział losowanych liczb.\
`fn rand(seed: &mut ..., min_rand: ..., max_rand: ...) -> ...`\
Każde losowanie oczywiście zmienia też ziarno.\
Możesz wybrać któryś z: https://en.wikipedia.org/wiki/Linear_congruential_generator \
wskazówka:
- zmien ziarno
- zwróć `ziarno%(max_rand-min_rand+1)+min_rand`
4. Napisz funkcję
`swap_arr(arr: ..., i: usize, j: usize)`
która zamieni wartości dwóch podanych elementów pewnej tablicy.
5. Stwórz funkcję\
`rand_perm(arr: ..., seed: ...)`\
permutującą w miejscu w sposób losowy wartości tablicy przekazanej w argumencie.\
**Uwaga:** Ta funkcja ma korzystać z dwóch poprzednich.