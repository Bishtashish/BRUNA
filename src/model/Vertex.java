package model;

import Util.Enums.WingType;
import navigation.Location;

public abstract class Vertex {
   private long node_id;
   private String callSign = "";
   private Health health;

   public Location getLoc() {
      return loc;
   }

   public void setLoc(Location loc) {
      this.loc = loc;
   }

   private Location loc;
   private long group_id;
   private WingType wingType;

   public WingType getWingType() {
      return wingType;
   }

   protected void setWingType(WingType wingType) {
      this.wingType = wingType;
   }



   public String getCallSign() {
      return callSign;
   }

   protected void setCallSign(String callSign) {
      this.callSign = callSign;
   }

   public Health getHealth() {
      return health;
   }

   protected void setHealth(Health health) {
      this.health = health;
   }

   public long getGroup_id() {
      return group_id;
   }

   protected void setGroup_id(long group_id) {
      this.group_id = group_id;
   }

   public long getNode_id() {
      return node_id;
   }

   protected void setNode_id(long node_id) {
      this.node_id = node_id;
   }



   abstract static class Builder<T extends Builder<T>>{
      //Required Parameters
      private  long node_id;
      private  long group_id;
      private Location loc;
      // Optional Parameters
      private String callSign = "";
      private Health health = null;
      private WingType wingType =  WingType.FIXED_WING;

      public T callSign(String val){
         callSign=val; return self() ;
      }
      public T health(Health val){
         health=val; return self();
      }
      public T wingType(WingType val){
         wingType=val; return self();
      }
      abstract Vertex build();
      // Subclass Must override
      protected abstract T self();
   }
   Vertex(Builder<?> builder){
      this.setNode_id(builder.node_id);
      this.setGroup_id(builder.group_id);
      this.setLoc(builder.loc);
      this.setCallSign(builder.callSign);
      this.setHealth(builder.health);
      this.setWingType(builder.wingType);
   }
}
