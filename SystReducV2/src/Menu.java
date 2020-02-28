import java.io.BufferedReader;
import java.io.File;
import java.io.IOException;
import java.nio.charset.Charset;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.nio.file.StandardOpenOption;
import java.text.DateFormat;
import java.text.DecimalFormat;
import java.text.SimpleDateFormat;
import java.util.*;

public class Menu {

	static DecimalFormat df = new DecimalFormat();

	public static void main(String[] args) {

		df.setMaximumFractionDigits(2);
		Scanner entree = new Scanner(System.in);
		String choix;
		// Avant toute chose, on vérifie que le répertoire de ressources est présent,
		// sinon on ne fait rien
		if (Files.notExists(Paths.get("Ressources"))) {
			System.out.println("ERREUR : DOSSIER RESSOURCES INTROUVABLE");
			System.exit(0);
		}

		// message de mise en garde et demande +18
		Menu.clear();
		Menu.avertissementMenu();

		choix = entree.nextLine();
		// On quite le programme si le message n'est pas accepté
		if (!choix.equalsIgnoreCase("oui") && !choix.equalsIgnoreCase("o")) {
			System.exit(0);
		}

		/// VARIABLES "GLOBALES"
		final int NB_NB_JOUABLES = 50; // pour generer une array de 1 a 50
		float margeMise = 1.15F;
		float prixGrille = 2.5F;
		int nbGarantis = 2; // garantie que veut l'utilisateur
		float miseMax = 0; // mise max de l'utilisteur
		List<Integer> reducteurNombres = new ArrayList<>(); // nombres de l'utilisateur
		List<Integer> list50 = new ArrayList<>(NB_NB_JOUABLES); // nombres ajoutables

		// generation des 50 nombres dispos
		for (int i = 1; i <= NB_NB_JOUABLES; i++) {
			list50.add(i);
		}

		List<Integer> nombresDispo = new ArrayList<>(list50);

		// boucle principale
		do {
			// variable de confirmation des actions de l'utilisateur
			Boolean confirm = null;
			// choix sur le menu principal
			switch (choix) {
			case "1":
				do {

					// Menu selection actions nombres du réducteur
					Menu.nombresSystemeMenu(reducteurNombres);
					System.out.println(">> Saisir une action :");
					choix = entree.nextLine();

					// choix ajout ou suppression
					switch (choix) {
					case "1":
						List<String> splitted;
						List<Integer> toModif;
						do {
							// Menu ajout nombres
							ajouterNombresSystemeMenu(reducteurNombres);
							if (confirm != null)
								if (confirm)
									System.out.println("|| + Nombre(s) ajouté(s)");
								else
									System.out.println("|| /!\\ Nombre(s) "
											+ "non ajouté(s)");
							System.out.println(" ");
							System.out.println(">> Saisir nombre(s) à ajouter");
							System.out.println("séparés par un espace "
									+ "ou une virgule (ou 0 pour quitter) :");
							choix = entree.nextLine();
							System.out.println(" ");
							if (!choix.equals("0")) {
								toModif = new ArrayList<>();
								confirm = null;
								splitted = new ArrayList<String>(
										Arrays.asList(choix.split(" |,")));
								for (String s : splitted) {
									try {
										Integer olala = Integer.parseInt(s);
										if (reducteurNombres.indexOf(olala) == -1
												&& nombresDispo.indexOf(olala) != -1
												&& !toModif.contains(olala)) {
											toModif.add(olala);
										} else {
											confirm = false;
										}
									} catch (Exception e) {
										confirm = false;
									}
								}
								if (confirm == null)
									confirm = true;
								Collections.sort(toModif);
								List<String> filtre = new ArrayList<>();
								for (Integer i : toModif) {
									filtre.add(String.valueOf(i));
								}
								splitted.removeAll(filtre);

								if (!confirm && toModif.size() > 0) {
									System.out.println("|| Ne peuvent pas être ajoutés : "
											+ splitted
											+ " Peuvent être ajoutés : " + toModif);
									System.out.println("|| Voulez-vous ajouter le(s) "
											+ " nombre(s) possible(s) ? (oui/non)");
									choix = entree.nextLine();
								}
								if (choix.equalsIgnoreCase("o")
										|| choix.equalsIgnoreCase("oui")
										|| confirm) {
									reducteurNombres.addAll(toModif);
									nombresDispo.removeAll(toModif);
									Collections.sort(reducteurNombres);
									confirm = true;
								}
								toModif = null;

							}

						} while (!choix.equals("0"));
						choix = "";
						confirm = null;
						break;

					case "2":
						do {
							// Menu suppression nombres
							supprimNombresSystemeMenu(reducteurNombres);
							if (confirm != null)
								if (confirm)
									System.out.println("|| - Nombre(s) supprimé(s)");
								else
									System.out.println("|| /!\\ Nombre(s) "
											+ "non supprimé(s)");
							System.out.println(" ");
							System.out.println(">> Saisir nombre(s) à supprimer");
							System.out.println("séparés par un espace ou une virgule"
									+ " (ou 0 pour quitter) :");
							choix = entree.nextLine();
							System.out.println(" ");
							if (!choix.equals("0")) {
								toModif = new ArrayList<>();
								confirm = null;
								splitted = new ArrayList<String>(
										Arrays.asList(choix.split(" |,")));
								for (String s : splitted) {
									try {
										Integer olala = Integer.parseInt(s);
										if (reducteurNombres.indexOf(olala) != -1
												&& nombresDispo.indexOf(olala) == -1
												&& !toModif.contains(olala)) {
											toModif.add(olala);
										} else {
											confirm = false;
										}

									} catch (Exception e) {
										confirm = false;
									}
								}
								if (confirm == null)
									confirm = true;
								Collections.sort(toModif);
								List<String> filtre = new ArrayList<>();
								for (Integer i : toModif) {
									filtre.add(String.valueOf(i));
								}
								splitted.removeAll(filtre);

								if (!confirm && toModif.size() > 0) {
									System.out.println("|| Ne peuvent pas "
											+ "être supprimé : "
													+ splitted
													+ " Peuvent être supprimé : "
													+ toModif);
									System.out.println("|| Voulez-vous supprimer "
											+ "le(s) possible(s) ? (oui/non)");
									choix = entree.nextLine();
								}
								if (choix.equalsIgnoreCase("o")
										|| choix.equalsIgnoreCase("oui")
										|| confirm) {
									reducteurNombres.removeAll(toModif);
									nombresDispo.addAll(toModif);
									Collections.sort(nombresDispo);
									confirm = true;
								}
								toModif = null;

							}
						} while (!choix.equals("0"));
						choix = "";
						confirm = null;
						break;

					case "3":
						do {
							// Menu tout supp
							Menu.clear();
							System.out.println(" ");
							System.out.println(">> Voulez-vous vraiment supprimer "
									+ "tous les nombres ? (oui/non)");
							System.out.println(" ");
							choix = entree.nextLine();
							System.out.println(" ");

							if (choix.equalsIgnoreCase("o")
									|| choix.equalsIgnoreCase("oui")) {
								reducteurNombres = new ArrayList<>();
								nombresDispo = new ArrayList<>(list50);
								System.out.println("Nombre(s) supprimé(s)");
							} else {
								System.out.println("Nombre(s) non supprimé(s)");
							}
							System.out.println(" ");
							System.out.println(">> Appuyez sur n'importe quelle touche "
									+ "pour revenir au menu précédent");
							choix = entree.nextLine();
							choix = "0";

						} while (!choix.equals("0"));
						choix = "";
						confirm = null;
						break;
					}

				} while (!choix.equals("0"));
				break;

			case "2":
				confirm = null;
				do {
					// menu garantie
					Menu.garantieMenu(nbGarantis);
					if (confirm != null)
						if (confirm)
							System.out.println("|| # Garantie modifiée");
						else
							System.out.println("|| /!\\ Nombre invalide");
					System.out.println(" ");
					System.out.println(">> Saisir la grantie désirée"
							+ " (ou 0 pour quitter) :");
					choix = entree.nextLine();
					if (!choix.equals("0")) {
						try {
							Integer nb = Integer.valueOf(choix);
							if (confirm = (nb > 1 && nb < 5))
								nbGarantis = nb;
						} catch (Exception e) {
							confirm = false;
						}
					}

				} while (!choix.equals("0"));
				break;

			case "3":
				confirm = null;
				do {
					// menu mise max
					Menu.miseMaxMenu(miseMax);
					if (confirm != null)
						if (confirm)
							System.out.println("|| # Mise max modifiée");
						else
							System.out.println("|| /!\\ Nombre invalide");
					System.out.println(" ");
					System.out.println(">> Saisir la mise max (ou 0 pour quitter) :");
					choix = entree.nextLine();
					if (!choix.equals("0")) {
						float nb;
						try {
							nb = Float.valueOf(choix);
							if (confirm = (nb > 0.0f && nb <= 5000.0f))
								miseMax = nb;
						} catch (Exception e) {
							confirm = false;
						}
					}

				} while (!choix.equals("0"));
				break;

			case "4":
				confirm = null;
				do {
					// menu marge mise
					Menu.miseMargeMaxMenu((margeMise - 1.0F) * 100.0F);
					if (confirm != null)
						if (confirm)
							System.out.println("|| # Marge mise modifiée");
						else
							System.out.println("|| /!\\ Nombre invalide");
					System.out.println(" ");
					System.out.println(">> Saisir la marge en pourcents(%)"
							+ " (ou 0 pour quitter) :");
					choix = entree.nextLine();
					if (!choix.equals("0")) {
						float nb;
						try {
							nb = Float.valueOf(choix);
							if (confirm = (nb >= 1.0f && nb <= 99999.0f))
								margeMise = (nb / 100.0F) + 1.0F;
						} catch (Exception e) {
							confirm = false;
						}
					}

				} while (!choix.equals("0"));
				break;

			case "5":
				do {
					// menu prix grille
					Menu.prixGrilleMenu(prixGrille);
					if (confirm != null)
						if (confirm)
							System.out.println("|| # Prix grille modifié");
						else
							System.out.println("|| /!\\ Nombre invalide");
					System.out.println(" ");
					System.out.println(">> Saisir un nouveau prix pour une grille"
							+ " (ou 0 pour quitter) :");
					choix = entree.nextLine();
					if (!choix.equals("0")) {
						float nb;
						try {
							nb = Float.valueOf(choix);
							if (confirm = (nb > 0.0f && nb <= 99999.0f))
								prixGrille = nb;
						} catch (Exception e) {
							confirm = false;
						}
					}

				} while (!choix.equals("0"));
				break;
			case "6":
					do {
						// suggestion parametres
						Menu.suggestionParamsMenu(margeMise, miseMax);
						System.out.println(">> Saisir une action :");
						choix = entree.nextLine();
					} while (!choix.equals("0"));
					break;

			case "9":
				do {
					// Menu de génération des grilles
					Menu.clear();
					// Vérification que l'utilisateur a saisie un système réducteur assez
					// grand
					if (reducteurNombres.size() < 11 || reducteurNombres.size() > 30
							|| (reducteurNombres.size() > 25 && nbGarantis == 4)) {
						System.out.println("=================================");
						System.out.println("/!\\ Impossible de générer les grilles, "
								+ "les paramètres saisis sont incorrects");
						System.out.println("=================================");
						System.out.println(" ");
						System.out.println("\n0 >> Revenir au menu");
						System.out.println(" ");
						choix = entree.nextLine();
					} else {

						final int TAILLESYSREDUC = reducteurNombres.size();
						final int GARANTIS = nbGarantis;

						// Code permettant de récupérer le chemin du bon fhichier
						String nFicLoad = "Ressources/data_";
						nFicLoad += TAILLESYSREDUC + "_" + GARANTIS + ".txt";
						Path path = Paths.get(nFicLoad);
						BufferedReader monFic = null;
						;
						try {
							monFic = Files.newBufferedReader(path,
									Charset.forName("UTF-8"));
						} catch (IOException e) {
							System.out.println("Erreur, fichier de données non trouvé");
							e.printStackTrace();
						}

						// code permettant de transformer les infos du fichier en objets
						// de type grille :
						List<Grille> grillesRecuperees = new ArrayList<>();
						String ligne = "";
						try {
							ligne = monFic.readLine();
						} catch (IOException e) {
							System.out.println("Erreur lecture ligne");
							e.printStackTrace();
						}
						while (ligne != null) {
							grillesRecuperees.add(new Grille(ligne));
							try {
								ligne = monFic.readLine();
							} catch (IOException e) {
								System.out.println("Erreur lecture ligne");
								e.printStackTrace();
							}
						}

						// Calcul du prix des grilles
						float prix = grillesRecuperees.size() * prixGrille;

						choix = "oui";
						// Vérification du respect de la mise maximum, avec
						// une marge de 15%
						if (prix > miseMax * margeMise) {
							Menu.prixDepasseMiseMenu(prix, miseMax, margeMise);
							choix = entree.nextLine();
						}

						if (choix.toLowerCase().equals("oui")) {
							// On fait la correspondance
							Outils.convertir(reducteurNombres, grillesRecuperees);
							// Trouver le chemin du fichier ou on va stoquer les
							// résultats, le fichier est
							// cree a partir de la date courante
							DateFormat format = new SimpleDateFormat(
									"yyyy_MM_dd-HH_mm_ss");
							Date dateCourante = new Date();
							// Vérification de la présence du dossier Resultats et
							// création le cas échéant
							Path folder = Paths.get("Resultats");
							if (Files.notExists(folder)) {
								File folde = new File("Resultats");
								boolean bool = folde.mkdir();
								if (bool) {
									System.out.println("Répertoire créée");
								} else {
									System.out.println("Impossible de "
											+ "créer le répertoire");
								}
							}
							// Ecriture des grilles dans les fichiers de resultat
							String nFic = "Resultats/";
							nFic += format.format(dateCourante) + ".txt";
							File f = new File(nFic);
							try {
								f.createNewFile();
							} catch (IOException e1) {
								e1.printStackTrace();
							}
							// Enregistrement des grilles
							List<String> strGrilles = new ArrayList<>();
							for (int i = 0; i < grillesRecuperees.size(); i++) {
								if (i > 0 && i % 5 == 0)
									strGrilles.add("");
								strGrilles.add(grillesRecuperees.get(i).toString());
							}
							Path fichier = Paths.get(nFic);
							try {
								Files.write(fichier, strGrilles, 
										Charset.forName("UTF-8"),
										StandardOpenOption.APPEND);
							} catch (IOException e) {
								e.printStackTrace();
							}
							// Affichahe des grilles jouées
							Menu.clear();
							for (int i = 0; i < grillesRecuperees.size(); i++) {
								if (i % 3 == 0 && i != 0)
									System.out.print("\n");
								if (i % 15 == 0 && i != 0)
									System.out.print("\n");
								System.out.print(grillesRecuperees.get(i) + " | ");
							}
							System.out.println("\n\n" + grillesRecuperees.size()
									+ " grilles générées dans le fichier " + nFic);
							System.out.println("\nMONTANT A "
									+ "MISER : " + prix + " euros\n");
							// Possibilite de retour au menu principal
							System.out.println("\n0 >> Revenir au menu");
							System.out.println(" ");
							choix = entree.nextLine();
						} else if (choix.toLowerCase().equals("non")) {
							choix = "0";
						}
					}
				} while (!choix.equals("0"));
				break;
			}

			// on affiche le menu principal
			Menu.accueilMenu(reducteurNombres.size(), nbGarantis, miseMax,
					(margeMise - 1.0F) * 100.0F, prixGrille);
			System.out.println(">> Saisir une action :");
			choix = entree.nextLine();

		} while (!choix.equals("0"));
		entree.close();

	}

	/**
	 * Vide l'affichage console
	 */
	private static void clear() {
		// methode dégeux de clear
		for (int y = 0; y < 50; y++)
			System.out.println("\n\n");
	}

	/**
	 * Affichage de l'avertissement
	 */
	private static void avertissementMenu() {
		System.out.println("/!\\ Attention /!\\");
		System.out.println(" ");
		System.out.println("Jouer comporte des risques : isolement, endettement...");
		System.out.println("Appelez le 09-74-75-13-13 (appel non surtaxé).");
		System.out.println(" ");
		System.out.println("Les jeux d'argent sont interdits aux mineurs,");
		System.out.println("en continuant vous confirmez avoir plus de 18 ans.");
		System.out.println(" ");
		System.out.println("Confirmez-vous avoir plus de 18 ans ? (oui/non)");
	}

	/**
	 * Affichage du menu d'accueil
	 * 
	 * @param taillesys taille du systeme reducteur
	 * @param garanti   nombre de nombres garantis
	 * @param mm        Mise maximale
	 */
	private static void accueilMenu(int taillesys, int garanti, float mm, float mm2, float pg) {
		Menu.clear();
		System.out.println("=================================");
		System.out.println("||        MENU PRINCIPAL       ||");
		System.out.println("=================================");
		System.out.println("|| # " + taillesys + " nombre(s) choisi(s).");
		System.out.println("|| # " + garanti + " nombre(s) garanti(s).");
		System.out.println("|| # Mise maximale définie : " + mm);
		System.out.println("|| # Marge mise définie : " + df.format(mm2) + "%");
		System.out.println("|| # Prix grille défini : " + pg + "euros.");
		System.out.println("||");
		System.out.println("|| 1 >> Choix des nombres du système");
		System.out.println("|| 2 >> Choix garantie");
		System.out.println("|| 3 >> Choix mise max");
		System.out.println("|| 4 >> Choix marge mise");
		System.out.println("|| 5 >> Choix prix grille");
		System.out.println("|| 6 >> Suggestion de paramètres selon mise");
		System.out.println("||");
		System.out.println("|| 9 >> Générer les grilles");
		Menu.finMenu();
	}

	/**
	 * Affichage du menu du choix des nombres du système réducteur
	 * 
	 * @param r liste des nombres choisis
	 */
	private static void nombresSystemeMenu(List<Integer> r) {
		Menu.clear();
		System.out.println("=================================");
		System.out.println("||      Nombres du système     ||");
		System.out.println("=================================");
		System.out.println("|| /!\\ Si vous souhaitez utiliser une garantie de 4");
		System.out.println("|| nombres, veuillez saisir au maximum 25 nombres");
		System.out.println("||");
		if (r.size() < 11 || r.size() > 30) {
			System.out.println("|| /!\\ Veuillez choisir entre 11 et 30 nombres");
			System.out.println("||");
		}
		System.out.println("|| # Vous avez choisi " + r.size() + " nombre(s).");
		System.out.println("|| # Qui sont : " + r);
		System.out.println("||");
		System.out.println("|| 1 >> Ajouter des nombres");
		System.out.println("|| 2 >> Supprimer des nombres");
		System.out.println("|| 3 >> Supprimer TOUS les nombres");
		Menu.finMenu();
	}

	/**
	 * Affichage du menu de supression de nombres
	 * 
	 * @param r liste des nombres choisis
	 */
	private static void ajouterNombresSystemeMenu(List<Integer> r) {
		Menu.clear();
		System.out.println("=================================");
		System.out.println("||       Ajouter nombres       ||");
		System.out.println("=================================");
		System.out.println("|| # Les nombres sont compris entre 1 et 50.");
		System.out.println("|| # Vous avez choisi " + r.size() + " nombre(s).");
		System.out.println("|| # Qui sont : " + r);
		System.out.println("||");
	}

	/**
	 * Affichage du menu de supression de nombres
	 * 
	 * @param r liste des nombres choisis
	 */
	private static void supprimNombresSystemeMenu(List<Integer> r) {
		Menu.clear();
		System.out.println("=================================");
		System.out.println("||      Supprimer nombres      ||");
		System.out.println("=================================");
		System.out.println("|| # Vous avez choisi " + r.size() + " nombre(s).");
		System.out.println("|| # Qui sont : " + r);
		System.out.println("||");
	}

	/**
	 * Affichage du menu du choix du nombres de nombres garantis
	 * 
	 * @param g nombre de nombres garantis
	 */
	private static void garantieMenu(int g) {
		Menu.clear();
		System.out.println("=================================");
		System.out.println("||       Nombres garantis      ||");
		System.out.println("=================================");
		System.out.println("|| # Vous aurez " + g + " nombres garantis.");
		System.out.println("|| # Garantie comprise entre 2 et 4.");
		System.out.println("||");
	}

	/**
	 * Affichage du menu de mise maximum
	 * 
	 * @param mm mise maximale
	 */
	private static void miseMaxMenu(float mm) {
		Menu.clear();
		System.out.println("=================================");
		System.out.println("||           Mise max          ||");
		System.out.println("=================================");
		System.out.println("|| # Mise maximale définie : " + mm + " euros");
		System.out.println("|| # Max : 5 000 euros.");
		System.out.println("||");
	}

	/**
	 * Affichage du menu de marge mise
	 * 
	 * @param mm marge mise
	 */
	private static void miseMargeMaxMenu(float mm) {

		Menu.clear();
		System.out.println("=================================");
		System.out.println("||          Marge mise         ||");
		System.out.println("=================================");
		System.out.println("|| # Marge mise définie : " + df.format(mm) + "%");
		System.out.println("|| # Marge doit être supérieure à 1%.");
		System.out.println("||");
	}

	/**
	 * Affichage de la fin commune à plusieurs menus
	 */
	private static void finMenu() {
		System.out.println("||");
		System.out.println("|| 0 >> Quitter");
		System.out.println("=================================");
		System.out.println(" ");
	}

	/**
	 * Affichage du menu de suggestion de parametres
	 */
	private static void suggestionParamsMenu(float coefTaux, float prix) {

		List<String> results = Outils.recupererParametres(coefTaux, prix);

		Menu.clear();
		System.out.println("=================================");
		System.out.println("||   Suggestion de paramètres  ||");
		System.out.println("=================================");
		if (results.size() > 0) {
			for (int i = 0; i < results.size(); i++) {
				System.out.println("||" + results.get(i));
			}
		} else {
			System.out.println("|| Aucun paramètre pour cette gamme de prix");
		}
		Menu.finMenu();
	}

	private static void prixDepasseMiseMenu(float p, float mm, float tx) {
		float txConv = (tx - 1) * 100;
		System.out.println("=================================");
		System.out.println("||     /!\\ Attention /!\\     ||");
		System.out.println("=================================");
		System.out.println("||");
		System.out.println("|| Avec les paramètres sélectionnés, la somme");
		System.out.println(
				"|| que vous devez miser dépasse de plus de " + df.format(txConv) + "%");
		System.out.println("|| la mise maximale que vous aviez indiquée.");
		System.out.println("|| Souhaitez-vous continuer ?");
		System.out.println("|| Prix estimé : " + p + "euros");
		System.out.println("|| Mise maximale saisie : " + mm + "euros");
		System.out.println("|| ");
		System.out.println("|| oui : continuer");
		System.out.println("|| non : retour au menu");
	}

	private static void prixGrilleMenu(float p) {
		Menu.clear();
		System.out.println("=================================");
		System.out.println("||         Prix Grille         ||");
		System.out.println("=================================");
		System.out.println("|| # Prix grille défini : " + p + "euros");
		System.out.println("|| # Prix d'une grille doit être supérieur à 0.0euros.");
		System.out.println("||");
	}

}
