## position

-  `static` 

    This keyword lets the element use the normal behavior, that is it is laid out in its current position in the flow. The top, right, bottom, left and  `z-index`  properties do not apply.

-  `relative` 

    This keyword lays out all elements as though the element were not positioned, and then adjust the element's position, without changing layout (and thus leaving a gap for the element where it would have been had it not been positioned). The effect of position:relative on  `table-*-group` , table-row, table-column, table-cell, and table-caption elements is undefined.

-  `absolute` 

    Do not leave space for the element. Instead, position it at a specified position relative to its closest positioned ancestor or to the containing block. Absolutely positioned boxes can have margins, they do not collapse with any other margins.

-  `fixed` 

    Do not leave space for the element. Instead, position it at a specified position relative to the screen's viewport and don't move it when scrolled. When printing, position it at that fixed position on every page. This value always create a new stacking context.

-  `sticky` 

    The box position is calculated according to the normal flow (this is called the position in normal flow). Then the box is offset relative to its flow root and containing block and in all cases, including table elements, does not affect the position of any following boxes. When a box B is stickily positioned, the position of the following box is calculated as though B were not offset. The effect of ‘position: sticky’ on table elements is the same as for ‘position: relative’.



## visibility

-  `visible` 

    Default value, the box is visible.

- `hidden` 

    The box is invisible (fully transparent, nothing is drawn), but still affects layout. Descendants of the element will be visible if they have visibility:visible (this doesn't work in IE up to version 7).

-   collapse

    For table rows, columns, column groups, and row groups the row(s) or column(s) are hidden and the space they would have occupied is removed (as if display: none were applied to the column/row of the table). However, the size of other rows and columns is still calculated as though the cells in the collapsed row(s) or column(s) are present. This was designed for fast removal of a row/column from a table without having to recalculate widths and heights for every portion of the table. For XUL elements, the computed size of the element is always zero, regardless of other styles that would normally affect the size, although margins still take effect. For other elements, collapse is treated the same as hidden.
