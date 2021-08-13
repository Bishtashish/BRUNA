package model;

public class Ear extends Node {
    private Object audio;

    public static class Builder extends Node.Builder{
        private Object audio;
        public Builder(Object val){
//          super(loc);
            this.audio=val;
        }
        @Override
        public Ear build(){ return new Ear(this);}
        @Override
        protected Builder self(){ return this;}

    }
    private Ear(Builder builder){
        super(builder);
        audio=builder.audio;
    }

}
