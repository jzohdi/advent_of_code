#include <iostream>
#include <vector>
#include <string>
#include <map>
#include "day_21.h"
#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "../external/doctest/doctest/doctest.h"

using namespace std;

// each player has a pawn
// starts at a random position 1-10
// each turn roll die 3 times -> add up the results
// move the pawn forward that many times
// increase player score by the value of the space their pawn stopped on
// game ends when a player reaches at least 1000
// players score starts at 0
struct DD
{
    int rolled_times;
    int last_rolled_num;
};
struct Player
{
    int score;
    int space;
};

int roll_die(DD *die)
{
    int roll1 = (die->last_rolled_num + 1) % 100;
    int roll2 = (die->last_rolled_num + 2) % 100;
    int roll3 = (die->last_rolled_num + 3) % 100;
    die->last_rolled_num = roll3;
    die->rolled_times += 3;
    return roll1 + roll2 + roll3;
}

int move_space(int curr_space, int num_moves)
{
    int new_space = (curr_space + num_moves) % 10;
    if (0 == new_space)
    {
        new_space = 10;
    }
    return new_space;
}

bool move(Player *player, DD *die)
{
    if (player->score >= 1000)
    {
        return false;
    }
    int total_moves = roll_die(die);
    int new_space = move_space(player->space, total_moves);
    player->score += new_space;
    player->space = new_space;
    if (player->score >= 1000)
    {
        return false;
    }
    return true;
}

int part_one(int player1_start, int player2_start)
{
    DD die;
    Player player1;
    Player player2;
    die.last_rolled_num = 0;
    die.rolled_times = 0;
    player1.score = 0;
    player1.space = player1_start;
    player2.score = 0;
    player2.space = player2_start;

    while (move(&player1, &die) && move(&player2, &die))
    {
        // cout << "player1 score: " << player1.score << " player2 score: " << player2.score << endl;
    }
    cout << "die rolled times: " << die.rolled_times << endl;
    int score = player1.score;
    if (player1.score > player2.score)
    {
        score = player2.score;
    }

    return score * die.rolled_times;
}

bool step_gen(tuple<int *, int *, int *> *gen)
{
    int *gen1 = get<0>(*gen);
    int *gen2 = get<1>(*gen);
    int *gen3 = get<2>(*gen);
    if (*gen1 >= 3 && *gen2 >= 3 && *gen3 >= 3)
    {
        return false;
    }
    *gen3 += 1;
    if (*gen3 == 4)
    {
        *gen3 = 1;
        *gen2 += 1;
    }
    if (*gen2 == 4)
    {
        *gen2 = 1;
        *gen1 += 1;
    }
    return true;
}

tuple<long long, long long> part_two_helper(int pos1, int score1, int pos2, int score2, map<string, tuple<long long, long long>> *cache)
{
    string key = to_string(pos1) + "," + to_string(score1) + "," + to_string(pos2) + "," + to_string(score2);
    if ((*cache).find(key) != (*cache).end())
    {
        return (*cache)[key];
    }
    long long wins1 = 0, wins2 = 0;
    int gen1 = 1, gen2 = 1, gen3 = 0;
    tuple<int *, int *, int *> gen = make_tuple(&gen1, &gen2, &gen3);
    while (step_gen(&gen))
    {
        int player1_dest = (pos1 + *get<0>(gen) + *get<1>(gen) + *get<2>(gen)) % 10;
        if (player1_dest % 10 == 0)
        {
            player1_dest = 10;
        }
        long long player1_score = score1 + player1_dest;
        if (player1_score >= 21)
        {
            wins1 += 1;
        }
        else
        {
            tuple<long long, long long> result = part_two_helper(pos2, score2, player1_dest, player1_score, cache);
            long long result2 = get<0>(result), result1 = get<1>(result);
            wins1 += result1;
            wins2 += result2;
        }
    }
    tuple<long long, long long> res = make_tuple(wins1, wins2);
    (*cache)[key] = res;
    return res;
}

unsigned long long part_two(int start1, int start2)
{
    map<string, tuple<long long, long long>> cache;
    tuple<long long, long long> result = (part_two_helper(start1, 0, start2, 0, &cache));
    long long res1 = get<0>(result);
    long long res2 = get<1>(result);
    if (res1 > res2)
    {
        return res1;
    }
    return res2;
}

TEST_CASE("Testing part one and two")
{
    // check example
    CHECK(part_one(4, 8) == 739785);
    // check part 1 sol
    int result = part_one(2, 8);
    cout << "resul part 1: " << result << endl;
    CHECK(result != 0);

    // part 2 example
    CHECK(part_two(4, 8) == 444356092776315);

    long long res2 = part_two(2, 8);
    cout << "result part2: " << res2 << endl;
}
