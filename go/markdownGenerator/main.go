package main

import (
	"fmt"
	"os"
	"strings"
)

type nodeKind int

const (
	Undefined nodeKind = iota
	Heading1
	Heading2
	BulletPoint
  // This is a good idea this could be the <ul> tag 
  BulletPointContainer
	Break
)

type node struct {
	kind    nodeKind
	content string
}

type treeNode struct {
	current node
	child   node
}

func main() {

	data, err := os.ReadFile("test.md")
	if err != nil {
		fmt.Println(err)
	}

	fileString := string(data)

	var lines []string = strings.Split(fileString, "\n")
	var nodes []node = make([]node, 0)
	for _, line := range lines {
		nodes = append(nodes, generateNode(line))
	}

	content := generateHtml(nodes)
	writeHtmlToFile(content)
}

func writeHtmlToFile(htmlContent string) {
	os.WriteFile("test.html", []byte(htmlContent), 0777)
}

func generateHtml(nodes []node) string {
	out := "<html><body>\n"
	for _, node := range nodes {
		fmt.Println(node)
		switch node.kind {
		case Undefined:
			out = out + "<p>" + node.content + "</p>"
			break
		case Heading1:
			out = out + "<h1>" + node.content + "</h1>"
			break
		case Heading2:
			out = out + "<h2>" + node.content + "</h2>"
			break
		case BulletPoint:
			out = out + "<li>" + node.content + "</li>"
			break
		case Break:
			out = out + "<hr>"
			break
		default:
			panic("help")
		}
		out = out + "\n"
	}
	out = out + "</body></html>"
	return out
}

func generateNode(line string) node {
	sign, content := strings.Split(line, " ")[0], strings.Split(line, " ")[1:]
	deKind := determineKind(sign)

	return node{kind: deKind, content: strings.Join(content, "")}
}

func determineKind(sign string) nodeKind {
	switch sign {
	case "#":
		return Heading1
	case "##":
		return Heading2
	case "-":
		return BulletPoint
	//TODO: i need to find a way to get any amount of minuses going for this
	case "---":
		return Break
	default:
		return Undefined
	}
}
