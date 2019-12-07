package deuxMilleDixHuit;

public class VariableBooleene extends ExpressionBooleenne {
	
	private String libelle;
	

	public String getLibelle() {
		return this.libelle;
	}

	public VariableBooleene(String lettre, String libelle) {
		this.symbole = lettre;
		this.libelle = libelle;
	}

	@Override
	public boolean evaluate() {
		return true;
	}

}
