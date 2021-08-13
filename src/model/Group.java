package model;

import patterns.DefaultPattern;
import patterns.Pattern;

import java.util.ArrayList;
import java.util.List;

public class Group {
    Pattern pattern;
    List<Head> headList;
    List<Node> nodeList;

    Group(){
        pattern = new DefaultPattern(headList,nodeList);
        headList = new ArrayList<Head>();
        nodeList = new ArrayList<Node>();
    }
}
