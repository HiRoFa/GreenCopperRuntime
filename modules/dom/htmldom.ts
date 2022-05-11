import * as grecoDom from 'greco://htmldom';

export type Node = {
    nodeValue: string,
    textContent: string,
    parentElement: null | Element,
    insertBefore: (node: Node, referenceNode: Node) => Node,
    insertAfter: (node: Node, referenceNode: Node) => Node,
    nodeType: number,
    ownerDocument: Document
};

export type NodeList = {
    length: number,
    item: (index: number) => Node,
    forEach: (callbackFn: (element: Node, index: number, list: NodeList), thisArg: any) => {}
};

export type ElementList = {
    length: number,
    item: (index: number) => Element,
};

export type TextNode = Node & {

};

export type Element = Node & {
    children: ElementList,
    childNodes: NodeList,

    firstChild: Node,
    lastChild: Node,

    previousSibling: null | Node,
    nextSibling: null | Node,
    nextElementSibling: null | Element,
    previousElementSibling: null | Element,

    getAttribute: (name: string) => null | string,
    setAttribute: (name: string, value: null | string) => void,

    innerHTML: string,
    outerHTML: string,

    className: null | string,
    localName: string,
    tagName: string,

    querySelector: (selectors: string) => null | Element,
    querySelectorAll: (selectors: string) => ElementList,

    appendChild: (node: Node) => Node,
    append: (...child: Array<string | Node>) => void,
    removeChild: (node: Node) => Node,
    replaceChild: (newChild: Node, oldChild: Node) => Node
};

export type Document = Element & {
    body: Element,
    documentElement: Element,
    createElement: (localName: string) => Element,
    createTextNode: (data: string) => TextNode,
    getElementById: (id: string) => Element
};

export type GrecoDOMParser {
    parseFromString: (html: string | Uint8Array, mimeType: string = "text/html") => Document,
    parseFromStringAsync: (html: string | Uint8Array, mimeType: string = "text/html") => Promise<Document>;
};

export const DOMParser: GrecoDOMParser = grecoDom.DOMParser;