# Leon

Useful information on [how browsers work](https://www.html5rocks.com/en/tutorials/internals/howbrowserswork/#CSS_parsing)

```
Flow:
HTML -> DOM Tree
                \
                 +-> Style Tree -> Layout Tree -> Painting
                /
CSS  -> Selectors/Rules
```

## Render Engine
### DOM
Create DOM tree nodes (constructors) from parsed html data. Right now,
two types of nodes are supported: Text and generic Element Nodes.

### Parser
Converts a text string to a DOM Tree. It reads in each element and converts
them into a tree of DOM nodes.

### Style
Creates another tree where each node contains:
- A reference to the dom node
- Style properties for a given node
- Child elemenets
