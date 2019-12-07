package deuxMilleDixHuit;

public abstract class ExpressionBooleenne {
	
	protected String symbole;
	
	public String getSymbole() {
		return this.symbole;
	}
	
	public abstract boolean evaluate();

}
