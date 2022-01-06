#include <iostream>
#include <vector>
#include <tuple>
#include <string>
#include <cmath>
#include "day_18.h"

using namespace std;

struct node
{
    int value;
    int parent;
    int id;
    int left;
    int right;
};

tuple<string, int> parse_next(string input, int *parent, vector<node *> *pairs);
void clean_up(vector<node *> *pairs);
string print_pairs(vector<node *> *pairs, int idx);
bool explode(node *curr, int inside_count, vector<node *> *pairs);
void add(int l_val, int r_val, int node_idx, vector<node *> *pairs);
bool split(node *curr, vector<node *> *pairs);

// [ [parent: -1, left: 1], [parent: 0, left: 2], [parent: 1, ]]
void run()
{
    string input_line = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]";
    vector<node *> pairs;
    int root_parent = -1;

    parse_next(input_line, &root_parent, &pairs);
    cout << "parsed pairs: " << print_pairs(&pairs, 0) << endl;
    explode(pairs[0], 0, &pairs);
    explode(pairs[0], 0, &pairs);
    split(pairs[0], &pairs);
    cout << "parsed pairs: " << print_pairs(&pairs, 0) << endl;

    clean_up(&pairs);

    cout << endl;
}

bool explode(node *curr, int inside_count, vector<node *> *pairs)
{
    int left_idx = curr->left;
    int right_idx = curr->right;
    if (left_idx == -1)
    {
        return false;
    }
    // replace self with 0
    if (inside_count == 4)
    {
        int left_value = (*pairs)[left_idx]->value;
        int right_value = (*pairs)[right_idx]->value;
        delete (*pairs)[left_idx];
        delete (*pairs)[right_idx];
        (*pairs)[left_idx] = nullptr;
        (*pairs)[right_idx] = nullptr;
        curr->value = 0;
        curr->left = -1;
        curr->right = -1;
        add(left_value, right_value, curr->id, pairs);
        return true;
    }
    return explode((*pairs)[left_idx], inside_count + 1, pairs) || explode((*pairs)[right_idx], inside_count + 1, pairs);
}

bool split(node *curr, vector<node *> *pairs)
{
    if (curr->left == -1)
    {
        if (curr->value >= 10)
        {
            int new_left = floor((float)curr->value / 2.0);
            int new_right = ceil((float)curr->value / 2.0);
            curr->value = -1;
            curr->left = (*pairs).size();
            curr->right = curr->left + 1;
            node *new_left_ptr = new node{
                new_left,
                curr->id,
                curr->left,
                -1,
                -1};
            node *new_right_ptr = new node{
                new_right,
                curr->id,
                curr->right,
                -1,
                -1};
            (*pairs).push_back(new_left_ptr);
            (*pairs).push_back(new_right_ptr);
            return true;
        }
        return false;
    }
    return split((*pairs)[curr->left], pairs) || split((*pairs)[curr->right], pairs);
}

void add_to_min(int val, int node_idx, vector<node *> *pairs)
{
    node *curr_node = (*pairs)[node_idx];
    if (curr_node->left == -1)
    {
        curr_node->value += val;
    }
    else
    {
        add_to_min(val, curr_node->left, pairs);
    }
}

void add_to_max(int val, int node_idx, vector<node *> *pairs)
{
    node *curr_node = (*pairs)[node_idx];
    if (curr_node->right == -1)
    {
        curr_node->value += val;
    }
    else
    {
        add_to_max(val, curr_node->right, pairs);
    }
}

void add(int l_val, int r_val, int node_idx, vector<node *> *pairs)
{
    node *curr_node = (*pairs)[node_idx];
    node *parent = (*pairs)[curr_node->parent];
    if (parent->left == curr_node->id)
    {
        add_to_min(r_val, parent->right, pairs);
        while (curr_node->parent != -1 && (*pairs)[curr_node->parent]->left == curr_node->id)
        {
            curr_node = (*pairs)[curr_node->parent];
        }
        if (curr_node->id != 0)
        {
            add_to_max(l_val, (*pairs)[curr_node->parent]->left, pairs);
        }
    }
    else
    {
        add_to_max(l_val, parent->left, pairs);
        while (curr_node->parent != -1 && (*pairs)[curr_node->parent]->right == curr_node->id)
        {
            curr_node = (*pairs)[curr_node->parent];
        }
        if (curr_node->id != 0)
        {
            add_to_min(r_val, (*pairs)[curr_node->parent]->right, pairs);
        }
    }
}

tuple<string, int> parse_next(string input, int *parent, vector<node *> *pairs)
{
    if (input.rfind("]", 0) == 0 || input.rfind(",", 0) == 0)
    {
        return parse_next(input.substr(1), parent, pairs);
    }

    if (input.rfind("[", 0) == 0)
    {
        node *new_node = new node{-1, -1, -1, -1, -1};
        new_node->parent = *parent;
        new_node->id = (*pairs).size();
        // int new_node_pos = (*pairs).size();
        (*pairs).push_back(new_node);

        tuple<string, int> result_1 = parse_next(input.substr(1), &new_node->id, pairs);
        string remainder = get<0>(result_1);
        int id_left = get<1>(result_1);
        new_node->left = id_left;

        tuple<string, int> result_2 = parse_next(remainder, &new_node->id, pairs);
        string rest = get<0>(result_2);
        int id_right = get<1>(result_2);
        new_node->right = id_right;
        return make_tuple(rest, new_node->id);
    }

    int value = input.at(0) - '0';
    node *new_node = new node{-1, -1, -1, -1, -1};
    new_node->value = value;
    new_node->parent = *parent;
    new_node->id = (*pairs).size();
    (*pairs).push_back(new_node);
    return make_tuple(input.substr(1), new_node->id);
}

string print_pairs(vector<node *> *pairs, int idx)
{
    if (idx >= (*pairs).size())
    {
        return "";
    }
    node *curr = (*pairs)[idx];
    if (curr->left == -1)
    {
        return to_string(curr->value);
    }

    return "[" + print_pairs(pairs, curr->left) + "," + print_pairs(pairs, curr->right) + "]";
}

void clean_up(vector<node *> *pairs)
{
    for (std::vector<node *>::size_type i = 0; i != (*pairs).size(); i++)
    {
        delete (*pairs)[i];
    }
}