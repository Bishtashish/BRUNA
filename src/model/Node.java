package model;

import Util.Enums.WingType;

public class Node extends Vertex{

    public static class Builder extends Vertex.Builder<Builder>{
        //Required Parameters
//        private final long node_id;
//        private final long group_id;
        // Optional Parameters
//        private String callSign = "";
//        private Health health = null;
//        private WingType wingType =  WingType.FIXED_WING;

//        public Builder(long node_id, long group_id ){
//            this.node_id=node_id;
//            this.group_id=group_id;
//        }
//        public Builder callSign( String val){
//            callSign=val; return this;
//        }
//        public Builder health(Health val){
//            health=val; return this;
//        }
//        public Builder wingType(WingType val){
//            wingType=val; return this;
//        }
        @Override
        public Node build(){
            return new Node(this);
        }
        @Override
        protected Builder self(){
            return this;
        }
    }
    protected Node(Builder builder){
        super(builder);
//        this.setNode_id(builder.node_id);
//        this.setGroup_id(builder.group_id);
//        this.setCallSign(builder.callSign);
//        this.setHealth(builder.health);
//        this.setWingType(builder.wingType);
    }
}
