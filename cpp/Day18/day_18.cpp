#include <iostream>
#include <vector>
#include <tuple>
#include <sstream>
#include <fstream>
#include <string>
#include <cmath>
#include "day_18.h"

using namespace std;

struct node
{
    int value;
    node *parent;
    node *left;
    node *right;
};

node *new_node(int val, node *p, node *l, node *r)
{
    return new node{val, p, l, r};
}

tuple<string, node *> parse_next(string input, node *parent);
void clean_up(node *curr);
string print_pairs(node *curr);
bool explode(node *curr, int inside_count);
void add_val(int l_val, int r_val, node *curr);
bool split(node *curr);
bool reduce(node *root);
node *add_nodes(node *left, node *right);
int magnitude(node *curr);

void part_one()
{
    string file_name = "../Day18/input.txt";
    ifstream infile(file_name);
    string line;
    node *root = NULL;
    while (getline(infile, line))
    {
        node *new_num = get<1>(parse_next(line, NULL));
        if (root == NULL)
        {
            root = new_num;
        }
        else
        {
            node *new_root = add_nodes(root, new_num);
            root = new_root;
            reduce(root);
        }
    }
    cout << "sum: " << magnitude(root) << endl;
    clean_up(root);
    root = NULL;
    cout << endl;
}

int mag_add(node *left, node *right)
{
    node *new_num = add_nodes(left, right);
    reduce(new_num);
    int mag = magnitude(new_num);
    clean_up(new_num);
    return mag;
}

void run()
{
    string file_name = "../Day18/input.txt";
    ifstream infile(file_name);
    string line;
    vector<node *> nodes;
    while (getline(infile, line))
    {
        node *new_num = get<1>(parse_next(line, NULL));
        nodes.push_back(new_num);
    }
    int max_sum = -1;
    for (int i = 0; i < nodes.size() - 1; i++)
    {
        for (int j = i + 1; j < nodes.size(); j++)
        {
            int mag = mag_add(nodes[i], nodes[j]);
            if (mag > max_sum)
            {
                max_sum = mag;
            }
            mag = mag_add(nodes[j], nodes[i]);
            if (mag > max_sum)
            {
                max_sum = mag;
            }
        }
    }
    for (int i = 0; i < nodes.size(); i++)
    {
        clean_up(nodes[i]);
    }
    cout << "max sum: " << max_sum << endl;
    cout << endl;
}

bool reduce(node *root)
{
    while (explode(root, 0) || split(root))
    {
    }
    return false;
}

bool explode(node *curr, int inside_count)
{
    node *left_child = curr->left;
    node *right_child = curr->right;
    if (left_child == NULL)
    {
        return false;
    }
    // replace self with 0
    if (inside_count == 4)
    {
        int left_value = left_child->value;
        int right_value = right_child->value;
        curr->value = 0;
        clean_up(left_child);
        clean_up(right_child);
        curr->left = NULL;
        curr->right = NULL;
        add_val(left_value, right_value, curr);
        return true;
    }
    return explode(left_child, inside_count + 1) || explode(right_child, inside_count + 1);
}

bool split(node *curr)
{
    if (curr->left == NULL)
    {
        if (curr->value >= 10)
        {
            int new_left = floor((float)curr->value / 2.0);
            int new_right = ceil((float)curr->value / 2.0);
            curr->value = -1;
            curr->left = new_node(new_left, curr, NULL, NULL);
            curr->right = new_node(new_right, curr, NULL, NULL);
            return true;
        }
        return false;
    }
    return split(curr->left) || split(curr->right);
}

void add_to_min(int val, node *curr)
{
    if (curr->left == NULL)
    {
        curr->value += val;
    }
    else
    {
        add_to_min(val, curr->left);
    }
}

void add_to_max(int val, node *curr)
{
    if (curr->right == NULL)
    {
        curr->value += val;
    }
    else
    {
        add_to_max(val, curr->right);
    }
}

void add_val(int l_val, int r_val, node *curr_node)
{
    node *parent = curr_node->parent;
    if (parent->left == curr_node)
    {
        add_to_min(r_val, parent->right);
        while (curr_node->parent != NULL && curr_node->parent->left == curr_node)
        {
            curr_node = curr_node->parent;
        }
        if (curr_node->parent != NULL)
        {
            add_to_max(l_val, curr_node->parent->left);
        }
    }
    else
    {
        add_to_max(l_val, parent->left);
        while (curr_node->parent != NULL && curr_node->parent->right == curr_node)
        {
            curr_node = curr_node->parent;
        }
        if (curr_node->parent != NULL)
        {
            add_to_min(r_val, curr_node->parent->right);
        }
    }
}

tuple<string, node *> parse_next(string input, node *parent)
{
    if (input.rfind("]", 0) == 0 || input.rfind(",", 0) == 0)
    {
        return parse_next(input.substr(1), parent);
    }

    if (input.rfind("[", 0) == 0)
    {
        node *new_node = new node{-1, parent, NULL, NULL};

        tuple<string, node *> result_1 = parse_next(input.substr(1), new_node);
        string remainder = get<0>(result_1);
        node *left_child = get<1>(result_1);
        new_node->left = left_child;

        tuple<string, node *> result_2 = parse_next(remainder, new_node);
        string rest = get<0>(result_2);
        node *right_child = get<1>(result_2);
        new_node->right = right_child;
        return make_tuple(rest, new_node);
    }

    int value = input.at(0) - '0';
    node *new_node = new node{value, parent, NULL, NULL};
    return make_tuple(input.substr(1), new_node);
}

node *deep_copy(node *root, node *parent)
{
    node *new_root = new node{root->value, parent, NULL, NULL};
    if (root->left != NULL)
    {
        new_root->left = deep_copy(root->left, new_root);
        new_root->right = deep_copy(root->right, new_root);
    }
    return new_root;
}

node *add_nodes(node *left, node *right)
{
    node *new_root = new node{-1, NULL, NULL, NULL};
    new_root->left = deep_copy(left, new_root);
    new_root->right = deep_copy(right, new_root);
    return new_root;
}

int magnitude(node *curr)
{
    if (curr->left == NULL)
    {
        return curr->value;
    }
    return 3 * magnitude(curr->left) + 2 * magnitude(curr->right);
}

string print_pairs(node *curr)
{
    if (curr == NULL)
    {
        return "";
    }
    if (curr->left == NULL)
    {
        return to_string(curr->value);
    }

    return "[" + print_pairs(curr->left) + "," + print_pairs(curr->right) + "]";
}

void collect(node *curr, vector<node *> *nodes)
{
    if (curr == NULL)
    {
        return;
    }
    curr->parent = NULL;
    (*nodes).push_back(curr);
    collect(curr->left, nodes);
    curr->left = NULL;
    collect(curr->right, nodes);
    curr->right = NULL;
}
void clean_up(node *root)
{
    vector<node *> flat_tree;
    collect(root, &flat_tree);
    for (std::vector<node *>::size_type i = 0; i != flat_tree.size(); i++)
    {
        delete flat_tree[i];
    }
}