import npeg

proc parser*() =
  let parser = peg("input"):
    input <- var_decl | print_stmt
    print_stmt <- "prt(" * value * ")"
    var_decl <- keyword * " " * name * " " * "=" * " " * value
    keyword <- "hui" | "piz"
    name <- +Alpha
    rule_int <- +Digit
    rule_bool <- "true" | "false"
    rule_float <- +Digit * "." * +Digit
    rule_string <- '"' * *(1 - '"') * '"'
    value <- rule_int | rule_bool | rule_float | rule_string
    