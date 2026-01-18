from creational.singleton.singleton import Configuration



def test_singleton_configuration():
    """Test para el patrón Singleton en Configuration"""
    config1 = Configuration()
    config2 = Configuration()

    assert config1 is config2, "Both instances should be the same"

    assert config1.program_name == "MyApp"
    assert config1.version == "1.0.0"

    # Modificar una instancia y verificar que la otra refleja el cambio
    config1.program_name = "YourApp"
    assert config2.program_name == "YourApp"