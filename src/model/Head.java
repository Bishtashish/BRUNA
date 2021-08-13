package model;

import feedback.Feed;

public class Head extends Node{
    private Feed feed;

    public static class Builder extends Node.Builder{
        private Feed feed;

        public Builder(Feed val){
            this.feed=val;
        }
        @Override
        public Head build(){ return new Head(this);}

        @Override
        protected Builder self(){ return this;}
    }
    private Head(Builder builder){
        super(builder);
        feed=builder.feed;
    }
}
