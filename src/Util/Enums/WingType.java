package Util.Enums;

public enum WingType {
     FIXED_WING, ROTARY_WINGS;
private WingType(){
    System.out.println("Constructor called : " + this.toString());
}
}
