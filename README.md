This approach is based on Fluent Localization System AST.

It uses a simplified Fluent AST - I removed everything except of nodes required to pass the examples.

It also differs in the area of how it handles selectors:

* I'm experimenting with `Option 2` of [Selectors](https://github.com/zbraniecki/message-format-2.0-rs/issues/6) discussion.
* I did not write any way to define "default" variants
* I am supporting multi-selectors based on Fluent's [flattening selectors](https://github.com/projectfluent/fluent/issues/4) proposal.

This approach allows us to maintain a simple, fairly powerful AST, but the lack of support for default variants is particularly glaring.

I believe we need to address handling of "default" variants for error recovery, and the decisions about it will have a profound impact
on how we will handle multi-selection variants.

The other area which is not currently handled is the area of [multi-variants with uneven branches](https://github.com/zbraniecki/message-format-2.0-rs/issues/9). The current approach does not feel
optimal for the problem described in the issue.
