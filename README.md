# Constituency tree parser
Parser created to parse constituency tree

### Description
This parser can be used to parse constituency tree into tree data structure. There are 2 types of tree nodes:
- Leaf node contains word and its tag
- Non-leaf node contains tag and 1 or more childrens

Tree can be used to determine sentence structure and find sentences with specific structure.

Example tree: (S (NP (PRP It)) (VP (VBZ is) (NP (NP (NN sentence)) (VP (VB to) (VP (VB test) (NP (NN constituencty) (NN tree)))))))
![Example tree](/assets/example.png)

### Usage

```
let tree = ConstituencyTree::parse(String::from("(S (NP (PRP It)) (VP (VBZ is) (NP (NP (NN sentence)) (VP (VB to) (VP (VB test) (NP (NN constituencty) (NN tree)))))))"));
```

Fuction return tree data structure, every node contains optional tag, value, and Vec of childrens