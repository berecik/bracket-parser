module.exports = grammar({
  name: 'bracket_parser',

  rules: {
    // The top-level rule is a source file, which can contain many items.
    source_file: $ => repeat($._item),

    // An item is either a piece of text or a bracketed expression.
    // The underscore makes it an "inlined" or "hidden" rule.
    _item: $ => choice(
      $.bracketed_expression,
      $.text
    ),

    // A bracketed expression can be any of our three types.
    bracketed_expression: $ => choice(
      $.paren_expression,
      $.square_expression,
      $.curly_expression
    ),

    // Define the structure for each bracket type.
    // They contain a sequence of more items, allowing for nesting.
    paren_expression: $ => seq('(', repeat($._item), ')'),
    square_expression: $ => seq('[', repeat($._item), ']'),
    curly_expression: $ => seq('{', repeat($._item), '}'),

    // The 'text' rule matches any sequence of one or more characters
    // that are NOT brackets.
    text: $ => /[^()\[\]{}]+/,
  }
});