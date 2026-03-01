import AST
import std/strformat
proc generate(node: Node): string =
    if node.kind == "print":
        result = fmt"echo {node.value}"