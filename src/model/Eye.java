package model;

public class Eye extends Node{
    private Object video;

    public static class Builder extends Node.Builder{
        private Object video;
        public Builder(Object val){
//            super();
            this.video=val;
        }
        @Override
        public Eye build(){
            return new Eye(this);
        }
        @Override
        protected Builder self(){
            return this;
        }


    }
    private Eye(Builder builder){
        super(builder);
        video=builder.video;
    }
}
