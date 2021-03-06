import { Paragraph } from "./paragraph";

export type VMergeType = "restart" | "continue";

export type VAlignType = "top" | "center" | "bottom";

export type CellProperty = {
  verticalMerge?: VMergeType;
  verticalAlign?: VAlignType;
  gridSpan?: number;
  width?: number;
};

export class TableCell {
  children: Paragraph[] = [];
  property: CellProperty = {};

  addParagraph(p: Paragraph) {
    this.children.push(p);
    return this;
  }

  verticalMerge(t: VMergeType) {
    this.property.verticalMerge = t;
    return this;
  }

  verticalAlign(t: VAlignType) {
    this.property.verticalAlign = t;
    return this;
  }

  gridSpan(v: number) {
    this.property.gridSpan = v;
    return this;
  }

  width(v: number) {
    this.property.width = v;
    return this;
  }
}
